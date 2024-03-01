use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SsItemId},
    shr::ModAggrMode,
    ss::{
        item::SsItem,
        svc::{
            calc::{
                attr::PENALTY_IMMUNE_CATS,
                support::{ModKey, Modification, SsAttrMod, SsAttrVal},
            },
            SsSvcs,
        },
        SsView,
    },
    util::Result,
};

impl SsSvcs {
    // Query methods
    pub(in crate::ss) fn calc_get_item_attr_val(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
        attr_id: &EAttrId,
    ) -> Result<SsAttrVal> {
        // Try accessing cached value
        match self.calc_data.attrs.get_item_attrs(item_id)?.get(attr_id) {
            Some(v) => return Ok(*v),
            _ => (),
        };
        // If it is not cached, calculate and cache it
        let val = self.calc_calc_item_attr_val(ss_view, item_id, attr_id)?;
        self.calc_data.attrs.get_item_attrs_mut(item_id)?.insert(*attr_id, val);
        Ok(val)
    }
    pub(in crate::ss) fn calc_get_item_attr_vals(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
    ) -> Result<HashMap<EAttrId, SsAttrVal>> {
        // SsItem can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let mut vals = self.calc_data.attrs.get_item_attrs_mut(item_id)?.clone();
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item
        for attr_id in ss_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            match self.calc_get_item_attr_val(ss_view, item_id, attr_id) {
                Ok(v) => vals.entry(*attr_id).or_insert(v),
                _ => continue,
            };
        }
        Ok(vals)
    }
    // Modification methods
    pub(in crate::ss::svc) fn calc_item_added(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_data.projs.item_added(item);
        self.handle_location_owner_change(ss_view, item);
    }
    pub(in crate::ss::svc) fn calc_item_removed(&mut self, ss_view: &SsView, item: &SsItem) {
        self.handle_location_owner_change(ss_view, item);
        self.calc_data.projs.item_removed(item);
    }
    pub(in crate::ss::svc) fn calc_item_loaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_data.attrs.add_item(item.get_id());
        self.calc_data.mods.reg_tgt(item, ss_view.fits);
        self.calc_data.projs.item_loaded(item);
    }
    pub(in crate::ss::svc) fn calc_item_unloaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_data.projs.item_unloaded(item);
        self.calc_data.mods.unreg_tgt(item, ss_view.fits);
        let item_id = item.get_id();
        self.calc_data.attrs.remove_item(&item_id);
        self.calc_data.caps.clear_item_caps(&item_id);
    }
    pub(in crate::ss::svc) fn calc_effects_started(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        let fit = item.get_fit_id().map(|v| ss_view.fits.get_fit(&v).ok()).flatten();
        let mods = generate_local_mods(item, effects);
        for modifier in mods.iter() {
            self.calc_data.mods.reg_mod(fit, *modifier);
        }
        for modifier in mods {
            for item_id in self
                .calc_data
                .mods
                .get_tgt_items(&modifier, ss_view.items, ss_view.fits)
            {
                self.calc_force_attr_recalc(ss_view, &item_id, &modifier.tgt_attr_id);
            }
        }
    }
    pub(in crate::ss::svc) fn calc_effects_stopped(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        let fit = item.get_fit_id().map(|v| ss_view.fits.get_fit(&v).ok()).flatten();
        let mods = generate_local_mods(item, effects);
        for modifier in mods.iter() {
            for item_id in self
                .calc_data
                .mods
                .get_tgt_items(&modifier, ss_view.items, ss_view.fits)
            {
                self.calc_force_attr_recalc(ss_view, &item_id, &modifier.tgt_attr_id);
            }
        }
        for modifier in mods.iter() {
            self.calc_data.mods.unreg_mod(fit, modifier);
        }
    }
    pub(in crate::ss::svc) fn calc_attr_value_changed(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
        attr_id: &EAttrId,
    ) {
        // Clear up attribute values which rely on passed attribute as an upper cap
        let capped_attr_ids = self
            .calc_data
            .caps
            .get_capped_attr_ids(item_id, attr_id)
            .map(|v| v.iter().map(|v| *v).collect_vec());
        if let Some(capped_attr_ids) = capped_attr_ids {
            for capped_attr_id in capped_attr_ids.iter() {
                self.calc_force_attr_recalc(ss_view, item_id, capped_attr_id);
            }
        };
        let mods = self
            .calc_data
            .mods
            .iter_mods_for_src(item_id)
            .filter(|v| v.src_attr_id == *attr_id)
            .map(|v| *v)
            .collect_vec();
        for modifier in mods.iter() {
            for tgt_item_id in self
                .calc_data
                .mods
                .get_tgt_items(&modifier, ss_view.items, ss_view.fits)
            {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &modifier.tgt_attr_id);
            }
        }
    }
    pub(in crate::ss) fn calc_force_attr_recalc(&mut self, ss_view: &SsView, item_id: &SsItemId, attr_id: &EAttrId) {
        match self.calc_data.attrs.get_item_attrs_mut(item_id) {
            Ok(item_attrs) => {
                if item_attrs.remove(attr_id).is_some() {
                    self.notify_attr_val_changed(ss_view, item_id, attr_id);
                }
            }
            _ => return,
        }
    }
    // Private methods
    pub(in crate::ss::svc::calc) fn calc_get_modifications(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        attr_id: &EAttrId,
    ) -> HashMap<ModKey, Modification> {
        let mut mods = HashMap::new();
        for modifier in self.calc_data.mods.get_mods_for_tgt(item, attr_id, ss_view.fits).iter() {
            let val = match self.calc_get_item_attr_val(ss_view, &modifier.src_item_id, &modifier.src_attr_id) {
                Ok(v) => v,
                _ => continue,
            };
            let src_item = match ss_view.items.get_item(&modifier.src_item_id) {
                Ok(i) => i,
                _ => continue,
            };
            let pen_immune = match src_item.get_category_id() {
                Ok(cid) => PENALTY_IMMUNE_CATS.contains(&cid),
                _ => continue,
            };
            // TODO: implement resistance support (add it to key as well? idk)
            let mod_key = ModKey::from(modifier);
            let modification = Modification::new(modifier.op, val.dogma, 1.0, ModAggrMode::Stack, pen_immune);
            mods.insert(mod_key, modification);
        }
        mods
    }
    fn handle_location_owner_change(&mut self, ss_view: &SsView, item: &SsItem) {
        if let Some(_) = item.get_top_domain() {
            for modifier in self
                .calc_data
                .mods
                .get_mods_for_changed_location_owner(item, ss_view.items)
            {
                for item_id in self
                    .calc_data
                    .mods
                    .get_tgt_items(&modifier, ss_view.items, ss_view.fits)
                {
                    self.calc_force_attr_recalc(ss_view, &item_id, &modifier.tgt_attr_id);
                }
            }
        }
    }
}

fn generate_local_mods(src_item: &SsItem, src_effects: &Vec<ad::ArcEffect>) -> Vec<SsAttrMod> {
    let mut mods = Vec::new();
    // Buff effects and system-wide effects do not have local modifiers by definition
    for effect in src_effects
        .iter()
        .filter(|e| !(e.is_system_wide | e.is_proj_buff | e.is_fleet_buff))
    {
        for a_mod in effect.mods.iter() {
            let ss_mod = SsAttrMod::from_a_data(src_item, effect, a_mod);
            mods.push(ss_mod);
        }
    }
    mods
}

fn generate_sw_mods(src_item: &SsItem, src_effects: &Vec<ad::ArcEffect>) -> Vec<SsAttrMod> {
    let mut mods = Vec::new();
    // Buff effects and system-wide effects do not have local modifiers by definition
    for effect in src_effects.iter().filter(|e| e.is_system_wide) {
        for a_mod in effect.mods.iter() {
            let ss_mod = SsAttrMod::from_a_data(src_item, effect, a_mod);
            mods.push(ss_mod);
        }
    }
    mods
}
