use crate::{
    misc::EffectSpec,
    svc::calc::{CtxModKey, CtxModifier, RawModId, RawModifier},
    util::{PSlab, RMapRSet},
};

/// General storage of raw modifiers.
#[derive(Clone)]
pub(in crate::svc::calc) struct RawModStorage {
    pub(super) data: PSlab<RawModId, RawModifier>,
    pub(super) by_espec: RMapRSet<EffectSpec, RawModId>,
}
impl RawModStorage {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            data: PSlab::with_capacity(0),
            by_espec: RMapRSet::new(),
        }
    }
    // Query methods
    pub(in crate::svc::calc) fn get_rmod(&self, rmod_id: RawModId) -> &RawModifier {
        self.data.get(rmod_id).unwrap()
    }
    pub(in crate::svc::calc) fn try_get_rmod(&self, rmod_id: RawModId) -> Option<&RawModifier> {
        self.data.get(rmod_id)
    }
    pub(in crate::svc::calc) fn get_cmod(&self, ckey: &CtxModKey) -> CtxModifier {
        CtxModifier::from_ckey(*self.get_rmod(ckey.rmod_id), *ckey)
    }
    pub(in crate::svc::calc) fn get_rmod_for_espec(
        &self,
        rmod_id: RawModId,
        espec: &EffectSpec,
    ) -> Option<RawModifier> {
        self.data
            .get(rmod_id)
            .filter(|rmod| rmod.affector_espec == *espec)
            .copied()
    }
    // Modification methods
    pub(in crate::svc::calc) fn add(&mut self, rmod: RawModifier) -> RawModId {
        let espec = rmod.affector_espec;
        let rmod_id = self.data.insert(rmod);
        self.by_espec.add_entry(espec, rmod_id);
        rmod_id
    }
    pub(in crate::svc::calc) fn add_for_espec(
        &mut self,
        reuse_rmod_ids: &mut Vec<RawModId>,
        espec: EffectSpec,
        rmods: &[RawModifier],
    ) {
        reuse_rmod_ids.clear();
        reuse_rmod_ids.extend(rmods.iter().map(|&rmod| self.data.insert(rmod)));
        self.by_espec.extend_entries(espec, reuse_rmod_ids.iter().copied());
    }
    pub(in crate::svc::calc) fn remove_full(&mut self, rmod_id: RawModId) {
        let rmod = self.data.remove(rmod_id);
        self.by_espec.remove_entry(rmod.affector_espec, &rmod_id);
    }
    pub(in crate::svc::calc) fn remove_by_espec_p1(&mut self, reuse_rmod_ids: &mut Vec<RawModId>, espec: &EffectSpec) {
        reuse_rmod_ids.clear();
        reuse_rmod_ids.extend(self.by_espec.remove_key(espec));
    }
    pub(in crate::svc::calc) fn remove_by_espec_p2(&mut self, rmod_id: RawModId) {
        self.data.remove(rmod_id);
    }
}
