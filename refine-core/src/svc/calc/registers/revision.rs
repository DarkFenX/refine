use crate::{
    svc::calc::{CtxModKey, CtxModifier, ItemAddRemoveReviser, RawModId},
    util::RSet,
};

/// Intended to hold modifiers which need special handling, e.g. custom AAR modifiers
#[derive(Clone)]
pub(in crate::svc::calc) struct RevisionRegister {
    // Item add/remove revisers are kept per kind, since a kind reacts to some item kinds only
    item_add_remove_aar_paste: RSet<CtxModKey>,
    item_add_remove_missile_flight_time: RSet<CtxModKey>,
}
impl RevisionRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            item_add_remove_aar_paste: RSet::new(),
            item_add_remove_missile_flight_time: RSet::new(),
        }
    }
    // Query methods
    pub(in crate::svc::calc) fn iter_revs_on_item_add_remove(
        &self,
        reviser: ItemAddRemoveReviser,
    ) -> impl ExactSizeIterator<Item = &CtxModKey> {
        self.get_item_add_remove(reviser).iter()
    }
    // Modification methods
    pub(in crate::svc::calc) fn reg_mod(&mut self, rmod_id: RawModId, cmod: &CtxModifier) {
        if let Some(reviser) = cmod.rmod.get_item_add_remove_reviser() {
            self.get_item_add_remove_mut(reviser).insert(cmod.get_ckey(rmod_id));
        }
    }
    pub(in crate::svc::calc) fn unreg_mod(&mut self, rmod_id: RawModId, cmod: &CtxModifier) {
        if let Some(reviser) = cmod.rmod.get_item_add_remove_reviser() {
            self.get_item_add_remove_mut(reviser).remove(&cmod.get_ckey(rmod_id));
        }
    }
    // Private methods
    fn get_item_add_remove(&self, reviser: ItemAddRemoveReviser) -> &RSet<CtxModKey> {
        match reviser {
            ItemAddRemoveReviser::AarPaste => &self.item_add_remove_aar_paste,
            ItemAddRemoveReviser::MissileFlightTime => &self.item_add_remove_missile_flight_time,
        }
    }
    fn get_item_add_remove_mut(&mut self, reviser: ItemAddRemoveReviser) -> &mut RSet<CtxModKey> {
        match reviser {
            ItemAddRemoveReviser::AarPaste => &mut self.item_add_remove_aar_paste,
            ItemAddRemoveReviser::MissileFlightTime => &mut self.item_add_remove_missile_flight_time,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, svc::calc::registers::RawModStorage, ud::UData};

    impl RevisionRegister {
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData, rmods: &RawModStorage) -> DebugResult {
            for reviser in ItemAddRemoveReviser::ALL {
                for ckey in self.get_item_add_remove(reviser).iter() {
                    ckey.consistency_check(u_data, rmods)?;
                }
            }
            Ok(())
        }
    }
}
