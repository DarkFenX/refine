//! Methods here reimplement attribute calculation counterparts to some extent, to provide extended
//! info while not bloating calculation part (since calculation is supposed to be used much more
//! often than modification info fetching).

use smallvec::SmallVec;

use super::calce_shared::get_base_attr_value;
use crate::{
    api::{AttrId, Op},
    misc::SecZone,
    rd::{RAttr, RAttrId},
    svc::{
        SvcCtx,
        calc::{Affector, AttrValInfo, Calc, CalcModification, CalcModificationKey, ModAccumInfo, Modification},
        err::KeyedItemLoadedError,
    },
    ud::{UItem, UItemId},
    util::{RMap, RMapVec, RSet, round},
};

struct Affection {
    modification: CalcModification,
    affectors: SmallVec<Affector, 1>,
}

impl Calc {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Query methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(in crate::svc) fn iter_item_mods(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemId,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, Vec<Modification>)> + use<>, KeyedItemLoadedError> {
        let mut info_map = RMapVec::new();
        for attr_key in self.iter_item_attr_keys(ctx, item_key)? {
            let mut attr_info = self.calc_item_attr_info(ctx, item_key, attr_key);
            let mut info_vec = Vec::new();
            info_vec.extend(attr_info.effective_infos.extract_if(.., |_| true));
            // info_vec.extend(attr_info.filtered_infos.extract_if(.., |_| true));
            if !info_vec.is_empty() {
                let attr_id = ctx.u_data.src.get_attr(attr_key).a_id.into();
                info_map.extend_entries(attr_id, info_vec.into_iter());
            }
        }
        Ok(info_map.into_iter())
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////
    // Private methods
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn iter_item_attr_keys(
        &self,
        ctx: SvcCtx,
        item_key: UItemId,
    ) -> Result<impl ExactSizeIterator<Item = RAttrId> + use<>, KeyedItemLoadedError> {
        let item_attr_data = self.get_item_data_with_err(item_key)?;
        let base_attrs = ctx.u_data.items.get(item_key).get_attrs().unwrap();
        let mut attr_keys = RSet::with_capacity(item_attr_data.len().max(base_attrs.len()));
        for (&attr_key, attr_entry) in item_attr_data.iter() {
            if attr_entry.value.is_some() {
                attr_keys.insert(attr_key);
            }
        }
        attr_keys.extend(base_attrs.keys().copied());
        Ok(attr_keys.into_iter())
    }
    fn iter_affections(
        &mut self,
        ctx: SvcCtx,
        item_key: &UItemId,
        item: &UItem,
        attr_key: RAttrId,
    ) -> impl Iterator<Item = Affection> {
        let mut affections = RMap::new();
        for cmod in self
            .std
            .get_mods_for_affectee(item_key, item, attr_key, &ctx.u_data.fits)
            .iter()
        {
            let val = match cmod.raw.get_mod_val(self, ctx) {
                Some(val) => val,
                None => continue,
            };
            let affector_item = ctx.u_data.items.get(cmod.raw.affector_espec.item_key);
            let affector_item_cat_id = affector_item.get_category_id().unwrap();
            let mod_key = CalcModificationKey::from(cmod);
            let modification = CalcModification {
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
    fn calc_item_attr_info(&mut self, ctx: SvcCtx, item_key: UItemId, attr_key: RAttrId) -> AttrValInfo {
        let item = ctx.u_data.items.get(item_key);
        let attr = ctx.u_data.src.get_attr(attr_key);
        let base_attr_info = self.calc_item_base_attr_info(ctx, item_key, item, attr);
        let mut accumulator = ModAccumInfo::new();
        for affection in self.iter_affections(ctx, &item_key, item, attr_key) {
            accumulator.add_val(
                affection.modification.val,
                affection.modification.proj_mult,
                affection.modification.res_mult,
                &affection.modification.op,
                attr.penalizable,
                &affection.modification.affector_item_cat_id,
                &affection.modification.aggr_mode,
                affection.affectors,
            );
        }
        let mut dogma_attr_info = accumulator.apply_dogma_mods(base_attr_info, attr.hig);
        // Lower value limit
        if let Some(limiter_attr_key) = attr.min_attr_key
            && let Ok(limiter_val) = self.get_item_attr_rfull(ctx, item_key, limiter_attr_key)
        {
            self.deps.add_anonymous(item_key, limiter_attr_key, attr_key);
            if limiter_val.dogma > dogma_attr_info.value {
                dogma_attr_info.value = limiter_val.dogma;
                dogma_attr_info.effective_infos.push(Modification {
                    op: Op::MinLimit,
                    initial_val: limiter_val.dogma,
                    range_mult: None,
                    resist_mult: None,
                    stacking_mult: None,
                    applied_val: limiter_val.dogma,
                    affectors: vec![Affector {
                        item_id: ctx.u_data.items.eid_by_iid(item_key),
                        attr_id: Some(ctx.u_data.src.get_attr(limiter_attr_key).a_id.into()),
                    }],
                })
            }
        }
        // Upper value limit
        if let Some(limiter_attr_key) = attr.max_attr_key
            && let Ok(limiter_val) = self.get_item_attr_rfull(ctx, item_key, limiter_attr_key)
        {
            self.deps.add_anonymous(item_key, limiter_attr_key, attr_key);
            if limiter_val.dogma < dogma_attr_info.value {
                dogma_attr_info.value = limiter_val.dogma;
                dogma_attr_info.effective_infos.push(Modification {
                    op: Op::MaxLimit,
                    initial_val: limiter_val.dogma,
                    range_mult: None,
                    resist_mult: None,
                    stacking_mult: None,
                    applied_val: limiter_val.dogma,
                    affectors: vec![Affector {
                        item_id: ctx.u_data.items.eid_by_iid(item_key),
                        attr_id: Some(ctx.u_data.src.get_attr(limiter_attr_key).a_id.into()),
                    }],
                })
            }
        }
        if ctx.ac().limited_precision.contains(&attr_key) {
            dogma_attr_info.value = round(dogma_attr_info.value, 2);
        }
        // Post-dogma calculations
        let extra_attr_info = accumulator.apply_extra_mods(dogma_attr_info, attr.hig);
        // Custom post-processing functions - since infos are not cached, it's fine to have it here
        match self.attrs.get_item_attr_data(&item_key).unwrap().get(&attr_key) {
            Some(attr_entry) if let Some(postprocs) = &attr_entry.postprocs => {
                let pp_fn = postprocs.info;
                pp_fn(self, ctx, item_key, extra_attr_info)
            }
            _ => extra_attr_info,
        }
    }
    fn calc_item_base_attr_info(&mut self, ctx: SvcCtx, item_key: UItemId, item: &UItem, attr: &RAttr) -> AttrValInfo {
        let attr_consts = ctx.ac();
        // Security modifier is a special case - it takes modified value of another attribute as its
        // own base
        if let Some(sec_zone_attr_key) = attr_consts.security_modifier
            && attr.r_id == sec_zone_attr_key
        {
            let security_attr_key = match ctx.u_data.sec_zone {
                SecZone::HiSec(_) => attr_consts.hisec_modifier,
                SecZone::LowSec(_) => attr_consts.lowsec_modifier,
                SecZone::NullSec | SecZone::WSpace | SecZone::Hazard => attr_consts.nullsec_modifier,
            };
            if let Some(security_attr_key) = security_attr_key
                && let Ok(security_full_val) = self.get_item_attr_rfull(ctx, item_key, security_attr_key)
            {
                // Ensure that change in any a security-specific attribute value triggers
                // recalculation of generic security attribute value
                self.deps.add_anonymous(item_key, security_attr_key, attr.r_id);
                let mut base_attr_info = AttrValInfo::new(security_full_val.dogma);
                base_attr_info.effective_infos.push(Modification {
                    // Technically this modification is not pre-assignment, it is base value
                    // overwrite (which later will be overwritten by any other pre-assignment
                    // regardless of its value), but pre-assignment is still used in info for
                    // simplicity. In any EVE scenario there is no pre-assignment for this attribute
                    op: Op::BaseAssign,
                    initial_val: security_full_val.dogma,
                    range_mult: None,
                    resist_mult: None,
                    stacking_mult: None,
                    applied_val: security_full_val.dogma,
                    affectors: vec![Affector {
                        item_id: ctx.u_data.items.eid_by_iid(item_key),
                        attr_id: Some(ctx.u_data.src.get_attr(security_attr_key).a_id.into()),
                    }],
                });
                return base_attr_info;
            }
        }
        AttrValInfo::new(get_base_attr_value(item, attr))
    }
}
