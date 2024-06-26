use crate::{sol::svc::svce_calc::SolCtxModifier, util::StSet};

// Intended to hold modifiers which need special handling, e.g. custom prop module modifiers
pub(in crate::sol::svc::svce_calc) struct SolRevisionRegister {
    pub(super) item_add: StSet<SolCtxModifier>,
    pub(super) item_remove: StSet<SolCtxModifier>,
}
impl SolRevisionRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            item_add: StSet::new(),
            item_remove: StSet::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_mods_on_item_add(&self) -> Vec<SolCtxModifier> {
        self.item_add.iter().map(|v| *v).collect()
    }
    pub(in crate::sol::svc::svce_calc) fn get_mods_on_item_remove(&self) -> Vec<SolCtxModifier> {
        self.item_remove.iter().map(|v| *v).collect()
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn reg_mod(&mut self, modifier: &SolCtxModifier) {
        if modifier.raw.needs_revision_on_item_add() {
            self.item_add.insert(*modifier);
        }
        if modifier.raw.needs_revision_on_item_remove() {
            self.item_remove.insert(*modifier);
        }
    }
    pub(in crate::sol::svc::svce_calc) fn unreg_mod(&mut self, modifier: &SolCtxModifier) {
        if modifier.raw.needs_revision_on_item_add() {
            self.item_add.remove(modifier);
        }
        if modifier.raw.needs_revision_on_item_remove() {
            self.item_remove.remove(modifier);
        }
    }
}
