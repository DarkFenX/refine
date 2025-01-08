use itertools::Itertools;

use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    err::basic::AttrMetaFoundError,
    sol::{
        svc::calc::{
            AttrCalcError, LoadedItemFoundError, SolAttrVal, SolCalc, SolModAccumFast, SolModification,
            SolModificationKey,
        },
        uad::{item::SolItem, SolUad},
    },
    util::{round, StMap},
};

const LIMITED_PRECISION_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::CPU,
    ec::attrs::POWER,
    ec::attrs::CPU_OUTPUT,
    ec::attrs::POWER_OUTPUT,
];

impl SolCalc {
    // Query methods
    pub(in crate::sol) fn get_item_attr_val(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrVal, AttrCalcError> {
        // Try accessing cached value
        let item_attr_data = self.attrs.get_item_attr_data(item_id)?;
        if let Some(val) = item_attr_data.values.get(attr_id) {
            return Ok(match item_attr_data.postprocs.get(attr_id) {
                Some(postprocs) => {
                    let pp_fn = postprocs.fast;
                    pp_fn(self, uad, item_id, *val)
                }
                None => *val,
            });
        }
        // If it is not cached, calculate and cache it
        let mut val = self.calc_item_attr_val(uad, item_id, attr_id)?;
        let item_attr_data = self.attrs.get_item_attr_data_mut(item_id).unwrap();
        item_attr_data.values.insert(*attr_id, val);
        if let Some(postprocs) = item_attr_data.postprocs.get(attr_id) {
            let pp_fn = postprocs.fast;
            val = pp_fn(self, uad, item_id, val);
        }
        Ok(val)
    }
    pub(in crate::sol::svc::calc) fn get_item_attr_val_no_pp(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrVal, AttrCalcError> {
        if let Some(val) = self.attrs.get_item_attr_data(item_id)?.values.get(attr_id) {
            return Ok(*val);
        };
        let val = self.calc_item_attr_val(uad, item_id, attr_id)?;
        self.attrs
            .get_item_attr_data_mut(item_id)
            .unwrap()
            .values
            .insert(*attr_id, val);
        Ok(val)
    }
    pub(in crate::sol) fn iter_item_attr_vals(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, SolAttrVal)>, LoadedItemFoundError> {
        let item = uad.items.get_item(item_id)?;
        // SolItem can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let item_attr_data = self.attrs.get_item_attr_data(item_id)?;
        let pp_attr_ids = item_attr_data.postprocs.keys().map(|v| *v).collect_vec();
        let mut vals = item_attr_data.values.clone();
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item
        for attr_id in item.get_attrs().unwrap().keys() {
            if let std::collections::hash_map::Entry::Vacant(entry) = vals.entry(*attr_id) {
                match self.get_item_attr_val(uad, &item.get_id(), attr_id) {
                    Ok(v) => entry.insert(v),
                    _ => continue,
                };
            }
        }
        for pp_attr_id in pp_attr_ids {
            if let Some(val) = vals.get(&pp_attr_id) {
                let pp_fn = self
                    .attrs
                    .get_item_attr_data(item_id)
                    .unwrap()
                    .postprocs
                    .get(&pp_attr_id)
                    .unwrap()
                    .fast;
                let val = pp_fn(self, uad, item_id, *val);
                vals.insert(pp_attr_id, val);
            }
        }
        Ok(vals.into_iter())
    }
    // Private methods
    fn iter_modifications(
        &mut self,
        uad: &SolUad,
        item: &SolItem,
        attr_id: &EAttrId,
    ) -> impl Iterator<Item = SolModification> {
        let mut mods = StMap::new();
        for modifier in self.std.get_mods_for_affectee(item, attr_id, &uad.fits).iter() {
            let val = match modifier.raw.get_mod_val(self, uad) {
                Some(v) => v,
                None => continue,
            };
            let affector_item = uad.items.get_item(&modifier.raw.affector_item_id).unwrap();
            let affector_item_cat_id = affector_item.get_category_id().unwrap();
            let mod_key = SolModificationKey::from(modifier);
            let modification = SolModification::new(
                modifier.raw.op,
                val,
                self.calc_resist_mult(uad, modifier),
                self.calc_proj_mult(uad, modifier),
                modifier.raw.aggr_mode,
                affector_item_cat_id,
            );
            mods.insert(mod_key, modification);
        }
        mods.into_values()
    }
    fn calc_item_attr_val(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrVal, AttrCalcError> {
        let item = uad.items.get_item(item_id)?;
        let attr = match uad.src.get_a_attr(attr_id) {
            Some(attr) => attr,
            None => return Err(AttrMetaFoundError::new(*attr_id).into()),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_attrs()?.get(attr_id) {
            Some(orig_val) => *orig_val,
            None => attr.def_val,
        };
        let mut accumulator = SolModAccumFast::new();
        for modification in self.iter_modifications(uad, item, attr_id) {
            accumulator.add_val(
                modification.val,
                modification.res_mult,
                modification.proj_mult,
                &modification.op,
                attr.penalizable,
                &modification.affector_item_cat_id,
                &modification.aggr_mode,
            );
        }
        let mut dogma_val = accumulator.apply_dogma_mods(base_val, attr.hig);
        // Lower value limit
        if let Some(limiter_attr_id) = attr.min_attr_id {
            if let Ok(limiter_val) = self.get_item_attr_val(uad, item_id, &limiter_attr_id) {
                self.deps.add_direct_local(*item_id, limiter_attr_id, *attr_id);
                dogma_val = AttrVal::max(dogma_val, limiter_val.dogma);
            }
        }
        // Upper value limit
        if let Some(limiter_attr_id) = attr.max_attr_id {
            if let Ok(limiter_val) = self.get_item_attr_val(uad, item_id, &limiter_attr_id) {
                self.deps.add_direct_local(*item_id, limiter_attr_id, *attr_id);
                dogma_val = AttrVal::min(dogma_val, limiter_val.dogma);
            }
        }
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_val = round(dogma_val, 2);
        }
        // Post-dogma calculations
        let extra_val = accumulator.apply_extra_mods(dogma_val, attr.hig);
        Ok(SolAttrVal::new(base_val, dogma_val, extra_val))
    }
}
