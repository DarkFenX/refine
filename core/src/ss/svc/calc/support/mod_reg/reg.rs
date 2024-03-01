use crate::{
    defs::{EAttrId, SsItemId},
    ss::{
        fit::{SsFit, SsFits},
        item::{SsItem, SsItems},
    },
};

use super::{super::modifier::SsAttrMod, mods::ModifierRegister, tgts::TgtItemRegister};

pub(in crate::ss::svc::calc) struct ModRegister {
    mods: ModifierRegister,
    tgts: TgtItemRegister,
}
impl ModRegister {
    pub(in crate::ss::svc::calc) fn new() -> Self {
        Self {
            mods: ModifierRegister::new(),
            tgts: TgtItemRegister::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::calc) fn get_tgt_items(
        &self,
        modifier: &SsAttrMod,
        items: &SsItems,
        fits: &SsFits,
    ) -> Vec<SsItemId> {
        self.tgts.get_tgt_items(modifier, items, fits)
    }
    pub(in crate::ss::svc::calc) fn get_mods_for_tgt(
        &self,
        tgt_item: &SsItem,
        tgt_attr_id: &EAttrId,
        fits: &SsFits,
    ) -> Vec<SsAttrMod> {
        self.mods.get_mods_for_tgt(tgt_item, tgt_attr_id, fits)
    }
    pub(in crate::ss::svc::calc) fn get_mods_for_changed_location_owner(
        &mut self,
        item: &SsItem,
        items: &SsItems,
    ) -> Vec<SsAttrMod> {
        self.mods.get_mods_for_changed_domain_owner(item, items)
    }
    pub(in crate::ss::svc::calc) fn iter_mods_for_src(
        &self,
        src_item_id: &SsItemId,
    ) -> impl Iterator<Item = &SsAttrMod> {
        self.mods.iter_mods_for_src(src_item_id)
    }
    // Modification methods
    pub(in crate::ss::svc::calc) fn reg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        self.tgts.reg_tgt(tgt_item, fits)
    }
    pub(in crate::ss::svc::calc) fn unreg_tgt(&mut self, tgt_item: &SsItem, fits: &SsFits) {
        self.tgts.unreg_tgt(tgt_item, fits)
    }
    pub(in crate::ss::svc::calc) fn reg_local_mod(&mut self, src_fit_opt: Option<&SsFit>, modifier: SsAttrMod) {
        self.mods.reg_mod(src_fit_opt, modifier)
    }
    pub(in crate::ss::svc::calc) fn unreg_local_mod(&mut self, src_fit_opt: Option<&SsFit>, modifier: &SsAttrMod) {
        self.mods.unreg_mod(src_fit_opt, modifier)
    }
}
