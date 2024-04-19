use crate::{ss::svc::svce_calc::SsAttrMod, util::StSet};

// Intended to hold modifiers which need special handling, e.g. custom prop module modifiers
pub(in crate::ss::svc::svce_calc) struct SsRevisionRegister {
    pub(super) item_add: StSet<SsAttrMod>,
    pub(super) item_remove: StSet<SsAttrMod>,
}
impl SsRevisionRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            item_add: StSet::new(),
            item_remove: StSet::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_mods_on_item_add(&self) -> Vec<SsAttrMod> {
        self.item_add.iter().map(|v| *v).collect()
    }
    pub(in crate::ss::svc::svce_calc) fn get_mods_on_item_remove(&self) -> Vec<SsAttrMod> {
        self.item_remove.iter().map(|v| *v).collect()
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn reg_mod(&mut self, ss_mod: SsAttrMod) {
        if ss_mod.needs_revision_on_item_add() {
            self.item_add.insert(ss_mod);
        }
        if ss_mod.needs_revision_on_item_remove() {
            self.item_remove.insert(ss_mod);
        }
    }
    pub(in crate::ss::svc::svce_calc) fn unreg_mod(&mut self, ss_mod: &SsAttrMod) {
        if ss_mod.needs_revision_on_item_add() {
            self.item_add.remove(ss_mod);
        }
        if ss_mod.needs_revision_on_item_remove() {
            self.item_remove.remove(ss_mod);
        }
    }
}
