use crate::{
    defs::{EAttrId, SsFitId, SsItemId},
    ss::{
        fit::{SsFit, SsFits},
        item::{SsItem, SsItems},
    },
};

use super::{super::super::modifier::SsAttrMod, mods::ModifierRegister, tgts::TgtItemRegister};

pub(in crate::ss::svc::svce_calc) struct ModRegister {
    mods: ModifierRegister,
    tgts: TgtItemRegister,
}
impl ModRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            mods: ModifierRegister::new(),
            tgts: TgtItemRegister::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_tgt_items(
        &self,
        modifier: &SsAttrMod,
        tgt_fits: &Vec<&SsFit>,
        items: &SsItems,
    ) -> Vec<SsItemId> {
        self.tgts.get_tgt_items(modifier, tgt_fits, items)
    }
    pub(in crate::ss::svc::svce_calc) fn get_mods_for_tgt(
        &self,
        tgt_item: &SsItem,
        tgt_attr_id: &EAttrId,
        fits: &SsFits,
    ) -> Vec<SsAttrMod> {
        self.mods.get_mods_for_tgt(tgt_item, tgt_attr_id, fits)
    }
    pub(in crate::ss::svc::svce_calc) fn get_mods_for_changed_location_owner(
        &mut self,
        item: &SsItem,
        items: &SsItems,
    ) -> Vec<SsAttrMod> {
        self.mods.get_mods_for_changed_domain_owner(item, items)
    }
    pub(in crate::ss::svc::svce_calc) fn iter_mods_for_src(
        &self,
        src_item_id: &SsItemId,
    ) -> impl Iterator<Item = &SsAttrMod> {
        self.mods.iter_mods_for_src(src_item_id)
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn reg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        self.tgts.reg_tgt(tgt_item, fits)
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        self.tgts.unreg_tgt(tgt_item, fits)
    }
    pub(in crate::ss::svc::svce_calc) fn reg_mod(&mut self, modifier: SsAttrMod) {
        self.mods.reg_mod(modifier)
    }
    pub(in crate::ss::svc::svce_calc) fn apply_mod(&mut self, modifier: SsAttrMod, tgt_fit_id_opt: Option<SsFitId>) {
        self.mods.apply_mod(modifier, tgt_fit_id_opt)
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_mod(&mut self, modifier: &SsAttrMod) {
        self.mods.unreg_mod(modifier)
    }
    pub(in crate::ss::svc::svce_calc) fn unapply_mod(&mut self, modifier: &SsAttrMod, tgt_fit_id_opt: Option<SsFitId>) {
        self.mods.unapply_mod(modifier, tgt_fit_id_opt)
    }
}
