use crate::{
    svc::calc::{CtxModKey, CtxModifier, ItemAddRemoveReviser, RawModId},
    util::RMap,
};

/// Intended to hold modifiers which need special handling, e.g. custom AAR modifiers
#[derive(Clone)]
pub(in crate::svc::calc) struct RevisionRegister {
    pub(super) item_add_remove: RMap<CtxModKey, ItemAddRemoveReviser>,
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
    ) -> impl ExactSizeIterator<Item = (&CtxModKey, &ItemAddRemoveReviser)> {
        self.item_add_remove.iter()
    }
    // Modification methods
    pub(in crate::svc::calc) fn reg_mod(&mut self, rmod_id: RawModId, cmod: &CtxModifier) {
        if let Some(item_add_reviser) = cmod.rmod.get_item_add_remove_reviser() {
            self.item_add_remove.insert(cmod.get_ckey(rmod_id), item_add_reviser);
        }
    }
    pub(in crate::svc::calc) fn unreg_mod(&mut self, rmod_id: RawModId, cmod: &CtxModifier) {
        if cmod.rmod.get_item_add_remove_reviser().is_some() {
            self.item_add_remove.remove(&cmod.get_ckey(rmod_id));
        }
    }
}
