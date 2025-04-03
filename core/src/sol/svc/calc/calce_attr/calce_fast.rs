use std::collections::hash_map::Entry;

use itertools::Itertools;

use crate::{
    ac, ad,
    err::basic::{AttrMetaFoundError, ItemLoadedError},
    sol::{
        AttrVal, ItemId, SecZone,
        svc::calc::{
            AttrCalcError, Calc, CalcAttrVal, LoadedItemFoundError, ModAccumFast, Modification, ModificationKey,
        },
        uad::{Uad, item::Item},
    },
    util::{HMap, round},
};

use super::calce_shared::{LIMITED_PRECISION_A_ATTR_IDS, get_base_attr_value};

impl Calc {
    // Query methods
    pub(in crate::sol) fn get_item_attr_val_extra_opt(
        &mut self,
        uad: &Uad,
        item_id: &Option<ItemId>,
        a_attr_id: &ad::AAttrId,
    ) -> Option<AttrVal> {
        item_id.and_then(|item_id| self.get_item_attr_val_extra(uad, &item_id, a_attr_id))
    }
    pub(in crate::sol) fn get_item_attr_val_extra(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) -> Option<AttrVal> {
        Some(self.get_item_attr_val_full(uad, item_id, a_attr_id).ok()?.extra)
    }
    pub(in crate::sol) fn get_item_attr_val_full(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, AttrCalcError> {
        // Try accessing cached value
        let item_attr_data = match self.attrs.get_item_attr_data(item_id) {
            Some(item_attr_data) => item_attr_data,
            // There can be no data due to one of two reasons: no item, or item is not loaded.
            // Figure which one is it
            None => {
                return Err(match uad.items.get_item(item_id) {
                    Ok(_) => ItemLoadedError { item_id: *item_id }.into(),
                    Err(error) => error.into(),
                });
            }
        };
        if let Some(cval) = item_attr_data.values.get(a_attr_id) {
            return Ok(match item_attr_data.postprocs.get(a_attr_id) {
                Some(postprocs) => {
                    let pp_fn = postprocs.fast;
                    pp_fn(self, uad, item_id, *cval)
                }
                None => *cval,
            });
        }
        // If it is not cached, calculate and cache it
        let mut cval = self.calc_item_attr_val(uad, item_id, a_attr_id)?;
        let item_attr_data = self.attrs.get_item_attr_data_mut(item_id).unwrap();
        item_attr_data.values.insert(*a_attr_id, cval);
        if let Some(postprocs) = item_attr_data.postprocs.get(a_attr_id) {
            let pp_fn = postprocs.fast;
            cval = pp_fn(self, uad, item_id, cval);
        }
        Ok(cval)
    }
    pub(in crate::sol::svc::calc) fn get_item_attr_val_no_pp(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, AttrCalcError> {
        let item_attr_data = match self.attrs.get_item_attr_data(item_id) {
            Some(item_attr_data) => item_attr_data,
            // There can be no data due to one of two reasons: no item, or item is not loaded.
            // Figure which one is it
            None => {
                return Err(match uad.items.get_item(item_id) {
                    Ok(_) => ItemLoadedError { item_id: *item_id }.into(),
                    Err(error) => error.into(),
                });
            }
        };
        if let Some(cval) = item_attr_data.values.get(a_attr_id) {
            return Ok(*cval);
        };
        let cval = self.calc_item_attr_val(uad, item_id, a_attr_id)?;
        self.attrs
            .get_item_attr_data_mut(item_id)
            .unwrap()
            .values
            .insert(*a_attr_id, cval);
        Ok(cval)
    }
    pub(in crate::sol) fn iter_item_attr_vals(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (ad::AAttrId, CalcAttrVal)>, LoadedItemFoundError> {
        let item = uad.items.get_item(item_id)?;
        // SolItem can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let item_attr_data = match self.attrs.get_item_attr_data(item_id) {
            Some(item_attr_data) => item_attr_data,
            None => return Err(ItemLoadedError { item_id: *item_id }.into()),
        };
        let pp_attr_ids = item_attr_data.postprocs.keys().copied().collect_vec();
        let mut cvals = item_attr_data.values.clone();
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item
        for attr_id in item.get_a_attrs().unwrap().keys() {
            if let Entry::Vacant(entry) = cvals.entry(*attr_id) {
                match self.get_item_attr_val_full(uad, &item.get_item_id(), attr_id) {
                    Ok(v) => entry.insert(v),
                    _ => continue,
                };
            }
        }
        for pp_attr_id in pp_attr_ids {
            if let Some(cval) = cvals.get(&pp_attr_id) {
                let pp_fn = self
                    .attrs
                    .get_item_attr_data(item_id)
                    .unwrap()
                    .postprocs
                    .get(&pp_attr_id)
                    .unwrap()
                    .fast;
                let cval = pp_fn(self, uad, item_id, *cval);
                cvals.insert(pp_attr_id, cval);
            }
        }
        Ok(cvals.into_iter())
    }
    // Private methods
    fn iter_modifications(
        &mut self,
        uad: &Uad,
        item: &Item,
        a_attr_id: &ad::AAttrId,
    ) -> impl Iterator<Item = Modification> {
        let mut mods = HMap::new();
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
            mods.insert(mod_key, modification);
        }
        mods.into_values()
    }
    fn calc_item_attr_val(
        &mut self,
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, AttrMetaFoundError> {
        let item = uad.items.get_item(item_id).unwrap();
        let a_attr = match uad.src.get_a_attr(a_attr_id) {
            Some(a_attr) => a_attr,
            None => return Err(AttrMetaFoundError { attr_id: *a_attr_id }),
        };
        // Get base value
        let base_val = match a_attr_id {
            // Security modifier is a special case - it takes modified value of another attribute as
            // its own base
            &ac::attrs::SECURITY_MODIFIER => {
                let security_a_attr_id = match uad.sec_zone {
                    SecZone::HiSec(_) => ac::attrs::HISEC_MODIFIER,
                    SecZone::LowSec(_) => ac::attrs::LOWSEC_MODIFIER,
                    _ => ac::attrs::NULLSEC_MODIFIER,
                };
                // Fetch base value for the generic attribute depending on solar system sec zone,
                // using its base value as a fallback
                match self.get_item_attr_val_full(uad, item_id, &security_a_attr_id) {
                    Ok(security_full_val) => {
                        // Ensure that change in any a security-specific attribute value triggers
                        // recalculation of generic security attribute value
                        self.deps.add_anonymous(*item_id, security_a_attr_id, *a_attr_id);
                        security_full_val.dogma
                    }
                    Err(_) => get_base_attr_value(item, a_attr),
                }
            }
            // Normal attributes
            _ => get_base_attr_value(item, a_attr),
        };
        // Get base value;
        let mut accumulator = ModAccumFast::new();
        for modification in self.iter_modifications(uad, item, a_attr_id) {
            accumulator.add_val(
                modification.val,
                modification.res_mult,
                modification.proj_mult,
                &modification.op,
                a_attr.penalizable,
                &modification.affector_a_item_cat_id,
                &modification.aggr_mode,
            );
        }
        let mut dogma_val = accumulator.apply_dogma_mods(base_val, a_attr.hig);
        // Lower value limit
        if let Some(limiter_attr_id) = a_attr.min_attr_id {
            if let Ok(limiter_cval) = self.get_item_attr_val_full(uad, item_id, &limiter_attr_id) {
                self.deps.add_anonymous(*item_id, limiter_attr_id, *a_attr_id);
                dogma_val = AttrVal::max(dogma_val, limiter_cval.dogma);
            }
        }
        // Upper value limit
        if let Some(limiter_attr_id) = a_attr.max_attr_id {
            if let Ok(limiter_cval) = self.get_item_attr_val_full(uad, item_id, &limiter_attr_id) {
                self.deps.add_anonymous(*item_id, limiter_attr_id, *a_attr_id);
                dogma_val = AttrVal::min(dogma_val, limiter_cval.dogma);
            }
        }
        if LIMITED_PRECISION_A_ATTR_IDS.contains(a_attr_id) {
            dogma_val = round(dogma_val, 2);
        }
        // Post-dogma calculations
        let extra_val = accumulator.apply_extra_mods(dogma_val, a_attr.hig);
        Ok(CalcAttrVal {
            base: base_val,
            dogma: dogma_val,
            extra: extra_val,
        })
    }
}
