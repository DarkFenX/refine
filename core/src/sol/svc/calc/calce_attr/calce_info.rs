//! Methods here reimplement attribute calculation counterparts to some extent, to provide extended
//! info while not bloating calculation part (since calculation is supposed to be used much more
//! often than modification info fetching).

use smallvec::SmallVec;

use crate::{
    ad,
    err::basic::AttrMetaFoundError,
    sol::{
        AttrVal, ItemId, OpInfo,
        svc::calc::{
            AffectorInfo, AttrCalcError, AttrValInfo, Calc, LoadedItemFoundError, ModAccumInfo, Modification,
            ModificationInfo, ModificationKey,
        },
        uad::{Uad, item::Item},
    },
    util::{StMap, StMapVecL1, StSet, round},
};

use super::calce_shared::LIMITED_PRECISION_A_ATTR_IDS;

struct Affection {
    modification: Modification,
    affectors: SmallVec<AffectorInfo, 1>,
}
impl Affection {
    fn new(modification: Modification, affectors: SmallVec<AffectorInfo, 1>) -> Self {
        Self {
            modification,
            affectors,
        }
    }
}

impl Calc {
    // Query methods
    pub(in crate::sol) fn iter_item_mods(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (ad::AAttrId, Vec<ModificationInfo>)>, LoadedItemFoundError> {
        let mut info_map = StMapVecL1::new();
        for a_attr_id in self.iter_item_a_attr_ids(uad, item_id)? {
            let mut attr_info = match self.calc_item_attr_info(uad, item_id, &a_attr_id) {
                Ok(attr_info) => attr_info,
                _ => continue,
            };
            let mut info_vec = Vec::new();
            info_vec.extend(attr_info.effective_infos.extract_if(.., |_| true));
            // info_vec.extend(attr_info.filtered_infos.extract_if(.., |_| true));
            if !info_vec.is_empty() {
                info_map.extend_entries(a_attr_id, info_vec.into_iter());
            }
        }
        Ok(info_map.into_iter())
    }
    // Private methods
    fn iter_item_a_attr_ids(
        &self,
        uad: &Uad,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = ad::AAttrId> + use<>, LoadedItemFoundError> {
        let mut a_attr_ids = StSet::new();
        for attr_id in uad.items.get_item(item_id)?.get_a_attrs_err()?.keys() {
            a_attr_ids.insert(*attr_id);
        }
        for attr_id in self.attrs.get_item_attr_data(item_id).unwrap().values.keys() {
            a_attr_ids.insert(*attr_id);
        }
        Ok(a_attr_ids.into_iter())
    }
    fn iter_affections(&mut self, uad: &Uad, item: &Item, a_attr_id: &ad::AAttrId) -> impl Iterator<Item = Affection> {
        let mut affections = StMap::new();
        for modifier in self.std.get_mods_for_affectee(item, a_attr_id, &uad.fits).iter() {
            let val = match modifier.raw.get_mod_val(self, uad) {
                Some(val) => val,
                None => continue,
            };
            let affector_item = uad.items.get_item(&modifier.raw.affector_item_id).unwrap();
            let affector_a_item_cat_id = affector_item.get_a_category_id().unwrap();
            let mod_key = ModificationKey::from(modifier);
            let modification = Modification {
                op: modifier.raw.op,
                val,
                res_mult: self.calc_resist_mult(uad, modifier),
                proj_mult: self.calc_proj_mult(uad, modifier),
                aggr_mode: modifier.raw.aggr_mode,
                affector_a_item_cat_id,
            };
            let affection = Affection::new(modification, modifier.raw.get_affector_info(uad));
            affections.insert(mod_key, affection);
        }
        affections.into_values()
    }
    fn calc_item_attr_info(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) -> Result<AttrValInfo, AttrCalcError> {
        let item = uad.items.get_item(item_id)?;
        let a_attr = match uad.src.get_a_attr(a_attr_id) {
            Some(a_attr) => a_attr,
            None => return Err(AttrMetaFoundError::new(*a_attr_id).into()),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_a_attrs_err()?.get(a_attr_id) {
            Some(orig_val) => *orig_val as AttrVal,
            None => a_attr.def_val as AttrVal,
        };
        let mut accumulator = ModAccumInfo::new();
        for affection in self.iter_affections(uad, item, a_attr_id) {
            accumulator.add_val(
                affection.modification.val,
                affection.modification.proj_mult,
                affection.modification.res_mult,
                &affection.modification.op,
                a_attr.penalizable,
                &affection.modification.affector_a_item_cat_id,
                &affection.modification.aggr_mode,
                affection.affectors,
            );
        }
        let mut dogma_attr_info = accumulator.apply_dogma_mods(base_val, a_attr.hig);
        // Lower value limit
        if let Some(limiter_a_attr_id) = a_attr.min_attr_id {
            if let Ok(limiter_val) = self.get_item_attr_val_full(uad, item_id, &limiter_a_attr_id) {
                self.deps.add_direct_local(*item_id, limiter_a_attr_id, *a_attr_id);
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
                            item_id: *item_id,
                            attr_id: Some(limiter_a_attr_id),
                        }],
                    })
                }
            }
        }
        // Upper value limit
        if let Some(limiter_a_attr_id) = a_attr.max_attr_id {
            if let Ok(limiter_val) = self.get_item_attr_val_full(uad, item_id, &limiter_a_attr_id) {
                self.deps.add_direct_local(*item_id, limiter_a_attr_id, *a_attr_id);
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
                            item_id: *item_id,
                            attr_id: Some(limiter_a_attr_id),
                        }],
                    })
                }
            }
        }
        if LIMITED_PRECISION_A_ATTR_IDS.contains(a_attr_id) {
            dogma_attr_info.value = round(dogma_attr_info.value, 2);
        }
        // Post-dogma calculations
        let extra_attr_info = accumulator.apply_extra_mods(dogma_attr_info, a_attr.hig);
        // Custom post-processing functions - since infos are not cached, it's fine to have it here
        let attr_info = match self.attrs.get_item_attr_data(item_id).unwrap().postprocs.get(a_attr_id) {
            Some(postprocs) => {
                let pp_fn = postprocs.info;
                pp_fn(self, uad, item_id, extra_attr_info)
            }
            None => extra_attr_info,
        };
        Ok(attr_info)
    }
}
