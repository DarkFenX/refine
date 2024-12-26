use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SolItemId},
    ec,
    sol::{
        item::SolItem,
        svc::{svce_calc::SolAttrVal, SolSvcs},
        SolView,
    },
    util::StMap,
};

// List all armor resonance attributes and also define default sorting order. When equal damage is
// received across several damage types, those which come earlier in this list will be picked as
// donors
const RES_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::ARMOR_EM_DMG_RESONANCE,
    ec::attrs::ARMOR_EXPL_DMG_RESONANCE,
    ec::attrs::ARMOR_KIN_DMG_RESONANCE,
    ec::attrs::ARMOR_THERM_DMG_RESONANCE,
];

impl SolSvcs {
    pub(in crate::sol::svc::svce_calc) fn calc_rah_effects_started(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        if self.calc_data.rah.running {
            return;
        }
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == ec::effects::ADAPTIVE_ARMOR_HARDENER) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                let other_item_ids = self.calc_data.rah.by_fit.get(&fit_id).map(|v| *v).collect_vec();
                for other_item_id in other_item_ids {
                    self.clear_results_for_item(sol_view, &other_item_id);
                }
                self.calc_data.rah.resonances.insert(item_id, StMap::new());
                self.calc_data.rah.by_fit.add_entry(fit_id, item_id);
                let item_attr_data = self.calc_data.attrs.get_item_attr_data_mut(&item_id).unwrap();
                item_attr_data
                    .postprocessors
                    .insert(ec::attrs::ARMOR_EM_DMG_RESONANCE, rah_em_resonance_postprocessor);
                item_attr_data
                    .postprocessors
                    .insert(ec::attrs::ARMOR_THERM_DMG_RESONANCE, rah_therm_resonance_postprocessor);
                item_attr_data
                    .postprocessors
                    .insert(ec::attrs::ARMOR_KIN_DMG_RESONANCE, rah_kin_resonance_postprocessor);
                item_attr_data
                    .postprocessors
                    .insert(ec::attrs::ARMOR_EXPL_DMG_RESONANCE, rah_expl_resonance_postprocessor);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn calc_rah_effects_stopped(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        if self.calc_data.rah.running {
            return;
        }
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == ec::effects::ADAPTIVE_ARMOR_HARDENER) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                let item_attr_data = self.calc_data.attrs.get_item_attr_data_mut(&item_id).unwrap();
                item_attr_data.postprocessors.remove(&ec::attrs::ARMOR_EM_DMG_RESONANCE);
                item_attr_data
                    .postprocessors
                    .remove(&ec::attrs::ARMOR_THERM_DMG_RESONANCE);
                item_attr_data
                    .postprocessors
                    .remove(&ec::attrs::ARMOR_KIN_DMG_RESONANCE);
                item_attr_data
                    .postprocessors
                    .remove(&ec::attrs::ARMOR_EXPL_DMG_RESONANCE);
                self.calc_data.rah.resonances.remove(&item_id);
                self.calc_data.rah.by_fit.remove_entry(&module.get_fit_id(), &item_id);
                let other_item_ids = self.calc_data.rah.by_fit.get(&fit_id).map(|v| *v).collect_vec();
                for other_item_id in other_item_ids {
                    self.clear_results_for_item(sol_view, &other_item_id);
                }
            }
        }
    }
    fn calc_rah_attr_value_changed(&mut self) {
        if self.calc_data.rah.running {
            return;
        }
        // Args: item ID, attr ID
        // Ship resistance attributes
        // - get item from view, if not ship do nothing, if ship go on
        // - clear results for all RAHs for ship's fit
        // RAH resistance attributes, shift amount
        // - check if item is in main storage, if no do nothing, if yes go on
        // - clear results for all items for its fit
        // RAH cycle time
        // - check if item is in main storage, if no do nothing, if yes go on
        // - if fit has only one RAH (check in fit-to-rah map), do nothing, if more go on
        // - clear results for all items for its fit
    }
    fn calc_rah_dmg_profile_changed(&mut self) {
        if self.calc_data.rah.running {
            return;
        }
    }
    // "Private" methods
    fn clear_results_for_item(&mut self, sol_view: &SolView, item_id: &SolItemId) {
        let rah_resos = self.calc_data.rah.resonances.get_mut(item_id).unwrap();
        if !rah_resos.is_empty() {
            rah_resos.clear();
            for attr_id in RES_ATTR_IDS.iter() {
                self.notify_attr_val_changed(sol_view, item_id, attr_id)
            }
        }
    }
}

fn rah_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    val: SolAttrVal,
    attr_id: EAttrId,
) -> SolAttrVal {
    val
}

fn rah_em_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    val: SolAttrVal,
) -> SolAttrVal {
    rah_resonance_postprocessor(svcs, sol_view, item_id, val, ec::attrs::ARMOR_EM_DMG_RESONANCE)
}

fn rah_therm_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    val: SolAttrVal,
) -> SolAttrVal {
    rah_resonance_postprocessor(svcs, sol_view, item_id, val, ec::attrs::ARMOR_THERM_DMG_RESONANCE)
}

fn rah_kin_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    val: SolAttrVal,
) -> SolAttrVal {
    rah_resonance_postprocessor(svcs, sol_view, item_id, val, ec::attrs::ARMOR_KIN_DMG_RESONANCE)
}

fn rah_expl_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    val: SolAttrVal,
) -> SolAttrVal {
    rah_resonance_postprocessor(svcs, sol_view, item_id, val, ec::attrs::ARMOR_EXPL_DMG_RESONANCE)
}
