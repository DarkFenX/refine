use crate::{
    misc::EffectSpec,
    svc::calc::{CtxModKey, CtxModifier, RawModId, RawModIdKind, RawModSlot, RawModTemplates, RawModifier},
    util::{PDenseRm, RMapRSet},
};

/// General storage of raw modifiers.
///
/// Modifiers which come from templates are not stored, only resolved here.
#[derive(Clone)]
pub(in crate::svc::calc) struct RawModStorage {
    data: PDenseRm<RawModSlot, RawModifier>,
    by_espec: RMapRSet<EffectSpec, RawModSlot>,
}
impl RawModStorage {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            data: PDenseRm::with_capacity(0),
            by_espec: RMapRSet::new(),
        }
    }
    // Query methods
    pub(in crate::svc::calc) fn get_rmod(&self, templates: &RawModTemplates, rmod_id: RawModId) -> RawModifier {
        match rmod_id.kind() {
            RawModIdKind::Template(template_id, item_uid) => templates.get(template_id, item_uid),
            RawModIdKind::Stored(slot) => *self.data.get(slot).unwrap(),
        }
    }
    pub(in crate::svc::calc) fn get_cmod(&self, templates: &RawModTemplates, ckey: &CtxModKey) -> CtxModifier {
        CtxModifier::from_ckey(self.get_rmod(templates, ckey.rmod_id), *ckey)
    }
    pub(in crate::svc::calc) fn get_rmod_for_espec(&self, slot: RawModSlot, espec: EffectSpec) -> Option<RawModifier> {
        self.data.get(slot).filter(|rmod| rmod.affector_espec == espec).copied()
    }
    // Modification methods
    pub(in crate::svc::calc) fn add(&mut self, rmod: RawModifier) -> RawModSlot {
        let espec = rmod.affector_espec;
        let slot = self.data.insert(rmod);
        self.by_espec.add_entry(espec, slot);
        slot
    }
    pub(in crate::svc::calc) fn add_for_espec(
        &mut self,
        reuse_rmod_slots: &mut Vec<RawModSlot>,
        espec: EffectSpec,
        rmods: &[RawModifier],
    ) {
        reuse_rmod_slots.clear();
        reuse_rmod_slots.extend(rmods.iter().map(|&rmod| self.data.insert(rmod)));
        self.by_espec.extend_entries(espec, reuse_rmod_slots.iter().copied());
    }
    pub(in crate::svc::calc) fn remove_full(&mut self, slot: RawModSlot) {
        let rmod = self.data.remove(slot);
        self.by_espec.remove_entry(rmod.affector_espec, &slot);
    }
    pub(in crate::svc::calc) fn remove_by_espec_p1(
        &mut self,
        reuse_rmod_slots: &mut Vec<RawModSlot>,
        espec: EffectSpec,
    ) {
        reuse_rmod_slots.clear();
        reuse_rmod_slots.extend(self.by_espec.remove_key(&espec));
    }
    pub(in crate::svc::calc) fn remove_by_espec_p2(&mut self, slot: RawModSlot) {
        self.data.remove(slot);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, ud::UData};

    impl RawModStorage {
        pub(in crate::svc::calc) fn try_get_rmod(
            &self,
            templates: &RawModTemplates,
            rmod_id: RawModId,
        ) -> Option<RawModifier> {
            match rmod_id.kind() {
                RawModIdKind::Template(template_id, item_uid) => templates.try_get(template_id, item_uid),
                RawModIdKind::Stored(slot) => self.data.get(slot).copied(),
            }
        }
        pub(in crate::svc::calc) fn iter_by_espec(
            &self,
        ) -> impl Iterator<Item = (&EffectSpec, impl Iterator<Item = &RawModifier>)> {
            self.by_espec
                .iter()
                .map(|(espec, slots)| (espec, slots.filter_map(|&slot| self.data.get(slot))))
        }
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
            let Self { data, by_espec } = self;
            for (slot, rmod) in data.iter() {
                // Every stored modifier has to be tracked by its effect, otherwise it has leaked
                if !by_espec.contains_entry(&rmod.affector_espec, &slot) {
                    return Err(Default::default());
                }
                rmod.consistency_check(u_data)?;
            }
            for (espec, slots) in by_espec.iter() {
                espec.consistency_check(u_data, true)?;
                // Modifiers can be registered only for running effects
                if !u_data
                    .items
                    .get(espec.item_uid)
                    .get_reffs()
                    .is_some_and(|reffs| reffs.contains(espec.effect_rid))
                {
                    return Err(Default::default());
                }
                for &slot in slots {
                    if data.get(slot).is_none() {
                        return Err(Default::default());
                    }
                }
            }
            Ok(())
        }
    }
}
