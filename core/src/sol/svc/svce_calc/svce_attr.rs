use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{SolAttrVal, SolAttrValues, SolModKey, SolModification},
            SolSvcs,
        },
        SolView,
    },
    util::{Error, ErrorKind, Result, StMap},
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
    ) -> Result<SolAttrVal> {
        // Try accessing cached value
        match self.calc_data.attrs.get_item_attrs(item_id)?.get(attr_id) {
            Some(v) => return Ok(*v),
            _ => (),
        };
        // If it is not cached, calculate and cache it
        let val = self.calc_calc_item_attr_val(sol_view, item_id, attr_id)?;
        self.calc_data.attrs.get_item_attrs_mut(item_id)?.insert(*attr_id, val);
        Ok(val)
    }
    pub(in crate::sol) fn calc_iter_item_attr_vals(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, SolAttrVal)>> {
        // SolItem can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let mut vals = self.calc_data.attrs.get_item_attrs_mut(item_id)?.clone();
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item
        for attr_id in sol_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            match self.calc_get_item_attr_val(sol_view, item_id, attr_id) {
                Ok(v) => vals.entry(*attr_id).or_insert(v),
                _ => continue,
            };
        }
        Ok(vals.into_iter())
    }
    // Private methods
    fn calc_get_modifications(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        attr_id: &EAttrId,
    ) -> StMap<SolModKey, SolModification> {
        let mut mods = StMap::new();
        for modifier in self
            .calc_data
            .mods
            .get_mods_for_tgt(item, attr_id, sol_view.fits)
            .iter()
        {
            let val = match modifier.get_mod_val(self, sol_view) {
                Ok(v) => v,
                _ => continue,
            };
            let src_item = match sol_view.items.get_item(&modifier.affector_item_id) {
                Ok(i) => i,
                _ => continue,
            };
            let src_item_cat_id = match src_item.get_category_id() {
                Ok(src_item_cat_id) => src_item_cat_id,
                _ => continue,
            };
            // TODO: implement resistance support (add it to key as well? idk)
            let mod_key = SolModKey::from(modifier);
            let modification = SolModification::new(modifier.op, val, 1.0, modifier.aggr_mode, src_item_cat_id);
            mods.insert(mod_key, modification);
        }
        mods
    }
    fn calc_calc_item_attr_val(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrVal> {
        let item = sol_view.items.get_item(item_id)?;
        let attr = match sol_view.src.get_a_attr(attr_id) {
            Some(attr) => attr,
            None => return Err(Error::new(ErrorKind::AAttrNotFound(*attr_id))),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_orig_attrs()?.get(attr_id) {
            Some(orig_val) => *orig_val,
            None => match attr.def_val {
                Some(def_val) => def_val,
                None => return Err(Error::new(ErrorKind::NoAttrBaseValue(*attr_id, item.get_a_item_id()))),
            },
        };
        match (attr_id, item) {
            (&ec::attrs::SKILL_LEVEL, SolItem::Skill(s)) => {
                return Ok(SolAttrVal::new(base_val, s.level as AttrVal, s.level as AttrVal))
            }
            _ => (),
        }
        let mut vals = SolAttrValues::new();
        for modification in self.calc_get_modifications(sol_view, item, attr_id).values() {
            vals.add_val(
                modification.val,
                &modification.op,
                attr.penalizable,
                &modification.src_item_cat_id,
                &modification.aggr_mode,
            );
        }
        let dogma_val = vals.apply_dogma_mods(base_val, attr.hig);
        // Upper cap for the attribute value being calculated
        let mut dogma_val = match attr.max_attr_id {
            Some(capping_attr_id) => match self.calc_get_item_attr_val(sol_view, item_id, &capping_attr_id) {
                Ok(capping_vals) => {
                    self.calc_data
                        .deps
                        .add_dependency(*item_id, capping_attr_id, *item_id, *attr_id);
                    AttrVal::min(dogma_val, capping_vals.dogma)
                }
                Err(_) => dogma_val,
            },
            None => dogma_val,
        };
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_val = (dogma_val * 100.0).round() / 100.0
        }
        // Post-dogma calculations
        let extra_val = vals.apply_extra_mods(dogma_val, attr.hig);
        Ok(SolAttrVal::new(base_val, dogma_val, extra_val))
    }
}
