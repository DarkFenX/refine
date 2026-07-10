use crate::{
    svc::calc::{CtxModifier, ItemAddRemoveReviser},
    util::RMap,
};

// Intended to hold modifiers which need special handling, e.g. custom AAR modifiers
#[derive(Clone)]
pub(in crate::svc::calc) struct RevisionRegister {
    pub(super) item_add_remove: RMap<CtxModifier, ItemAddRemoveReviser>,
}
impl RevisionRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            item_add_remove: RMap::new(),
        }
    }
    // Query methods
    pub(in crate::svc::calc) fn iter_revs_on_item_add_remove(
        &self,
    ) -> impl ExactSizeIterator<Item = (&CtxModifier, &ItemAddRemoveReviser)> {
        self.item_add_remove.iter()
    }
    // Modification methods
    pub(in crate::svc::calc) fn reg_mod(&mut self, cmod: &CtxModifier) {
        if let Some(item_add_reviser) = cmod.raw.get_item_add_remove_reviser() {
            self.item_add_remove.insert(*cmod, item_add_reviser);
        }
    }
    pub(in crate::svc::calc) fn unreg_mod(&mut self, cmod: &CtxModifier) {
        if cmod.raw.get_item_add_remove_reviser().is_some() {
            self.item_add_remove.remove(cmod);
        }
    }
}
