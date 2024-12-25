use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    err::basic::AttrMetaFoundError,
    sol::{
        item::SolItem,
        svc::{
            err::{AttrCalcError, LoadedItemFoundError},
            svce_calc::{SolAttrVal, SolModAccumFast, SolModification, SolModificationKey},
            SolSvcs,
        },
        SolView,
    },
    util::StMap,
};

const LIMITED_PRECISION_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::CPU,
    ec::attrs::POWER,
    ec::attrs::CPU_OUTPUT,
    ec::attrs::POWER_OUTPUT,
];

impl SolSvcs {
    // Query methods
    pub(in crate::sol) fn calc_get_item_attr_val(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrVal, AttrCalcError> {
        // Try accessing cached value
        if let Some(val) = self.calc_data.attrs.get_item_attr_data(item_id)?.values.get(attr_id) {
            return Ok(*val);
        }
        // If it is not cached, calculate and cache it
        let val = self.calc_calc_item_attr_val(sol_view, item_id, attr_id)?;
        self.calc_data
            .attrs
            .get_item_attr_data_mut(item_id)
            .unwrap()
            .values
            .insert(*attr_id, val);
        Ok(val)
    }
    pub(in crate::sol) fn calc_iter_item_attr_vals(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, SolAttrVal)>, LoadedItemFoundError> {
        let item = sol_view.items.get_item(item_id)?;
        // SolItem can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let mut vals = self.calc_data.attrs.get_item_attr_data(&item.get_id())?.values.clone();
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item
        for attr_id in item.get_attrs().unwrap().keys() {
            match self.calc_get_item_attr_val(sol_view, &item.get_id(), attr_id) {
                Ok(v) => vals.entry(*attr_id).or_insert(v),
                _ => continue,
            };
        }
        Ok(vals.into_iter())
    }
    // Private methods
    fn calc_iter_modifications(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        attr_id: &EAttrId,
    ) -> impl Iterator<Item = SolModification> {
        let mut mods = StMap::new();
        for modifier in self
            .calc_data
            .std
            .get_mods_for_affectee(item, attr_id, sol_view.fits)
            .iter()
        {
            let val = match modifier.raw.get_mod_val(self, sol_view) {
                Some(v) => v,
                None => continue,
            };
            let affector_item = sol_view.items.get_item(&modifier.raw.affector_item_id).unwrap();
            let affector_item_cat_id = affector_item.get_category_id().unwrap();
            let mod_key = SolModificationKey::from(modifier);
            let modification = SolModification::new(
                modifier.raw.op,
                val,
                self.calc_resist_mult(sol_view, modifier),
                self.calc_proj_mult(sol_view, modifier),
                modifier.raw.aggr_mode,
                affector_item_cat_id,
            );
            mods.insert(mod_key, modification);
        }
        mods.into_values()
    }
    fn calc_calc_item_attr_val(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrVal, AttrCalcError> {
        let item = sol_view.items.get_item(item_id)?;
        let attr = match sol_view.src.get_a_attr(attr_id) {
            Some(attr) => attr,
            None => return Err(AttrMetaFoundError::new(*attr_id).into()),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_attrs()?.get(attr_id) {
            Some(orig_val) => *orig_val,
            None => attr.def_val,
        };
        match (attr_id, item) {
            (&ec::attrs::SKILL_LEVEL, SolItem::Skill(s)) => {
                return Ok(SolAttrVal::new(
                    base_val,
                    s.get_level() as AttrVal,
                    s.get_level() as AttrVal,
                ))
            }
            _ => (),
        }
        let mut accumulator = SolModAccumFast::new();
        for modification in self.calc_iter_modifications(sol_view, item, attr_id) {
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
            if let Ok(limiter_val) = self.calc_get_item_attr_val(sol_view, item_id, &limiter_attr_id) {
                self.calc_data
                    .deps
                    .add_direct_local(*item_id, limiter_attr_id, *attr_id);
                dogma_val = AttrVal::max(dogma_val, limiter_val.dogma);
            }
        }
        // Upper value limit
        if let Some(limiter_attr_id) = attr.max_attr_id {
            if let Ok(limiter_val) = self.calc_get_item_attr_val(sol_view, item_id, &limiter_attr_id) {
                self.calc_data
                    .deps
                    .add_direct_local(*item_id, limiter_attr_id, *attr_id);
                dogma_val = AttrVal::min(dogma_val, limiter_val.dogma);
            }
        }
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_val = (dogma_val * 100.0).round() / 100.0
        }
        // Post-dogma calculations
        let extra_val = accumulator.apply_extra_mods(dogma_val, attr.hig);
        Ok(SolAttrVal::new(base_val, dogma_val, extra_val))
    }
}
