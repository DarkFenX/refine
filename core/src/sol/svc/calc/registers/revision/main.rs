use crate::{sol::svc::calc::CtxModifier, util::HSet};

// Intended to hold modifiers which need special handling, e.g. custom prop module modifiers
#[derive(Clone)]
pub(in crate::sol::svc::calc) struct RevisionRegister {
    pub(super) item_add: HSet<CtxModifier>,
    pub(super) item_remove: HSet<CtxModifier>,
}
impl RevisionRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self {
            item_add: HSet::new(),
            item_remove: HSet::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::calc) fn iter_mods_on_item_add(&self) -> impl ExactSizeIterator<Item = &CtxModifier> {
        self.item_add.iter()
    }
    pub(in crate::sol::svc::calc) fn iter_mods_on_item_remove(&self) -> impl ExactSizeIterator<Item = &CtxModifier> {
        self.item_remove.iter()
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn reg_mod(&mut self, modifier: &CtxModifier) {
        if modifier.raw.needs_revision_on_item_add() {
            self.item_add.insert(*modifier);
        }
        if modifier.raw.needs_revision_on_item_remove() {
            self.item_remove.insert(*modifier);
        }
    }
    pub(in crate::sol::svc::calc) fn unreg_mod(&mut self, modifier: &CtxModifier) {
        if modifier.raw.needs_revision_on_item_add() {
            self.item_add.remove(modifier);
        }
        if modifier.raw.needs_revision_on_item_remove() {
            self.item_remove.remove(modifier);
        }
    }
}
