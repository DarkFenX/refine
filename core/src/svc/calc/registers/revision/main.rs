use crate::{
    svc::calc::{CtxModifier, ItemAddReviser, ItemRemoveReviser},
    util::RMap,
};

// Intended to hold modifiers which need special handling, e.g. custom prop module modifiers
#[derive(Clone)]
pub(in crate::svc::calc) struct RevisionRegister {
    pub(super) item_add: RMap<CtxModifier, ItemAddReviser>,
    pub(super) item_remove: RMap<CtxModifier, ItemRemoveReviser>,
}
impl RevisionRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            item_add: RMap::new(),
            item_remove: RMap::new(),
        }
    }
    // Query methods
    pub(in crate::svc::calc) fn iter_revs_on_item_add(
        &self,
    ) -> impl ExactSizeIterator<Item = (&CtxModifier, &ItemAddReviser)> {
        self.item_add.iter()
    }
    pub(in crate::svc::calc) fn iter_revs_on_item_remove(
        &self,
    ) -> impl ExactSizeIterator<Item = (&CtxModifier, &ItemRemoveReviser)> {
        self.item_remove.iter()
    }
    // Modification methods
    pub(in crate::svc::calc) fn reg_mod(&mut self, modifier: &CtxModifier) {
        if let Some(item_add_reviser) = modifier.raw.get_item_add_reviser() {
            self.item_add.insert(*modifier, item_add_reviser);
        }
        if let Some(item_remove_reviser) = modifier.raw.get_item_remove_reviser() {
            self.item_remove.insert(*modifier, item_remove_reviser);
        }
    }
    pub(in crate::svc::calc) fn unreg_mod(&mut self, modifier: &CtxModifier) {
        if modifier.raw.get_item_add_reviser().is_some() {
            self.item_add.remove(modifier);
        }
        if modifier.raw.get_item_remove_reviser().is_some() {
            self.item_remove.remove(modifier);
        }
    }
}
