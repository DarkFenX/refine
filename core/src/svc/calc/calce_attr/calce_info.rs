//! Methods here reimplement attribute calculation counterparts to some extent, to provide extended
//! info while not bloating calculation part (since calculation is supposed to be used much more
//! often than modification info fetching).

use smallvec::SmallVec;

use super::calce_shared::{LIMITED_PRECISION_ATTR_IDS, get_base_attr_value, make_default_attr};
use crate::{
    ac,
    ad::AAttrId,
    misc::{OpInfo, SecZone},
    svc::{
        SvcCtx,
        calc::{AffectorInfo, AttrValInfo, Calc, ModAccumInfo, Modification, ModificationInfo, ModificationKey},
        err::KeyedItemLoadedError,
    },
    ud::{UItem, UItemKey},
    util::{RMap, RMapVec, RSet, round},
};

struct Affection {
    modification: Modification,
    affectors: SmallVec<AffectorInfo, 1>,
}

impl Calc {
    // Query methods
    pub(in crate::svc) fn iter_item_mods(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
    ) -> Result<impl ExactSizeIterator<Item = (AAttrId, Vec<ModificationInfo>)> + use<>, KeyedItemLoadedError> {
        let mut info_map = RMapVec::new();
        for attr_id in self.iter_item_attr_ids(ctx, item_key)? {
            let mut attr_info = self.calc_item_attr_info(ctx, item_key, &attr_id);
            let mut info_vec = Vec::new();
            info_vec.extend(attr_info.effective_infos.extract_if(.., |_| true));
            // info_vec.extend(attr_info.filtered_infos.extract_if(.., |_| true));
            if !info_vec.is_empty() {
                info_map.extend_entries(attr_id, info_vec.into_iter());
            }
        }
        Ok(info_map.into_iter())
    }
    // Private methods
    fn iter_item_attr_ids(
        &self,
        ctx: SvcCtx,
        item_key: UItemKey,
    ) -> Result<impl ExactSizeIterator<Item = AAttrId> + use<>, KeyedItemLoadedError> {
        let item_attrs = match ctx.u_data.items.get(item_key).get_attrs() {
            Some(item_a_attrs) => item_a_attrs,
            None => return Err(KeyedItemLoadedError { item_key }),
        };
        let mut attr_ids: RSet<_> = item_attrs.keys().copied().collect();
        attr_ids.extend(self.attrs.get_item_attr_data(&item_key).unwrap().values.keys().copied());
        Ok(attr_ids.into_iter())
    }
    fn iter_affections(
        &mut self,
        ctx: SvcCtx,
        item_key: &UItemKey,
        item: &UItem,
        attr_id: &AAttrId,
    ) -> impl Iterator<Item = Affection> {
        let mut affections = RMap::new();
        for cmod in self
            .std
            .get_mods_for_affectee(item_key, item, attr_id, &ctx.u_data.fits)
            .iter()
        {
            let val = match cmod.raw.get_mod_val(self, ctx) {
                Some(val) => val,
                None => continue,
            };
            let affector_item = ctx.u_data.items.get(cmod.raw.affector_espec.item_key);
            let affector_item_cat_id = affector_item.get_category_id().unwrap();
            let mod_key = ModificationKey::from(cmod);
            let modification = Modification {
                op: cmod.raw.op,
                val,
                res_mult: self.calc_resist_mult(ctx, cmod),
                proj_mult: self.calc_proj_mult(ctx, cmod),
                aggr_mode: cmod.raw.aggr_mode,
                affector_item_cat_id,
            };
            let affection = Affection {
                modification,
                affectors: cmod.raw.get_affector_info(ctx),
            };
            affections.insert(mod_key, affection);
        }
        affections.into_values()
    }
    fn calc_item_attr_info(&mut self, ctx: SvcCtx, item_key: UItemKey, attr_id: &AAttrId) -> AttrValInfo {
        let item = ctx.u_data.items.get(item_key);
        let attr = match ctx.u_data.src.get_attr(attr_id) {
            Some(attr) => attr,
            None => &make_default_attr(*attr_id),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute
        // value.
        let base_attr_info = match attr_id {
            &ac::attrs::SECURITY_MODIFIER => {
                // Fetch base value for the generic attribute depending on solar system sec zone,
                // using its base value as a fallback
                let security_attr_id = match ctx.u_data.sec_zone {
                    SecZone::HiSec(_) => ac::attrs::HISEC_MODIFIER,
                    SecZone::LowSec(_) => ac::attrs::LOWSEC_MODIFIER,
                    SecZone::NullSec | SecZone::WSpace | SecZone::Hazard => ac::attrs::NULLSEC_MODIFIER,
                };
                match self.get_item_attr_val_full(ctx, item_key, &security_attr_id) {
                    Ok(security_full_val) => {
                        // Ensure that change in any a security-specific attribute value triggers
                        // recalculation of generic security attribute value
                        self.deps.add_anonymous(item_key, security_attr_id, *attr_id);
                        let mut base_attr_info = AttrValInfo::new(security_full_val.dogma);
                        base_attr_info.effective_infos.push(ModificationInfo {
                            // Technically this modification is not pre-assignment, it is base value
                            // overwrite (which later will be overwritten by any other
                            // pre-assignment regardless of its value), but pre-assignment is still
                            // used in info for simplicity. In any EVE scenario there is no
                            // pre-assignment for this attribute
                            op: OpInfo::BaseAssign,
                            initial_val: security_full_val.dogma,
                            range_mult: None,
                            resist_mult: None,
                            stacking_mult: None,
                            applied_val: security_full_val.dogma,
                            affectors: vec![AffectorInfo {
                                item_id: ctx.u_data.items.id_by_key(item_key),
                                attr_id: Some(security_attr_id),
                            }],
                        });
                        base_attr_info
                    }
                    Err(_) => AttrValInfo::new(get_base_attr_value(item, attr)),
                }
            }
            _ => AttrValInfo::new(get_base_attr_value(item, attr)),
        };
        let mut accumulator = ModAccumInfo::new();
        for affection in self.iter_affections(ctx, &item_key, item, attr_id) {
            accumulator.add_val(
                affection.modification.val,
                affection.modification.proj_mult,
                affection.modification.res_mult,
                &affection.modification.op,
                attr.is_penalizable(),
                &affection.modification.affector_item_cat_id,
                &affection.modification.aggr_mode,
                affection.affectors,
            );
        }
        let mut dogma_attr_info = accumulator.apply_dogma_mods(base_attr_info, attr.is_hig());
        // Lower value limit
        if let Some(limiter_attr_id) = attr.get_min_attr_id()
            && let Ok(limiter_val) = self.get_item_attr_val_full(ctx, item_key, &limiter_attr_id)
        {
            self.deps.add_anonymous(item_key, limiter_attr_id, *attr_id);
            if limiter_val.dogma > dogma_attr_info.value {
                dogma_attr_info.value = limiter_val.dogma;
                dogma_attr_info.effective_infos.push(ModificationInfo {
                    op: OpInfo::MinLimit,
                    initial_val: limiter_val.dogma,
                    range_mult: None,
                    resist_mult: None,
                    stacking_mult: None,
                    applied_val: limiter_val.dogma,
                    affectors: vec![AffectorInfo {
                        item_id: ctx.u_data.items.id_by_key(item_key),
                        attr_id: Some(limiter_attr_id),
                    }],
                })
            }
        }
        // Upper value limit
        if let Some(limiter_attr_id) = attr.get_max_attr_id()
            && let Ok(limiter_val) = self.get_item_attr_val_full(ctx, item_key, &limiter_attr_id)
        {
            self.deps.add_anonymous(item_key, limiter_attr_id, *attr_id);
            if limiter_val.dogma < dogma_attr_info.value {
                dogma_attr_info.value = limiter_val.dogma;
                dogma_attr_info.effective_infos.push(ModificationInfo {
                    op: OpInfo::MaxLimit,
                    initial_val: limiter_val.dogma,
                    range_mult: None,
                    resist_mult: None,
                    stacking_mult: None,
                    applied_val: limiter_val.dogma,
                    affectors: vec![AffectorInfo {
                        item_id: ctx.u_data.items.id_by_key(item_key),
                        attr_id: Some(limiter_attr_id),
                    }],
                })
            }
        }
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_attr_info.value = round(dogma_attr_info.value, 2);
        }
        // Post-dogma calculations
        let extra_attr_info = accumulator.apply_extra_mods(dogma_attr_info, attr.is_hig());
        // Custom post-processing functions - since infos are not cached, it's fine to have it here
        match self.attrs.get_item_attr_data(&item_key).unwrap().postprocs.get(attr_id) {
            Some(postprocs) => {
                let pp_fn = postprocs.info;
                pp_fn(self, ctx, item_key, extra_attr_info)
            }
            None => extra_attr_info,
        }
    }
}
