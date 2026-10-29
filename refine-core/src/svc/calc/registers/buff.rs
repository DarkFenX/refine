use crate::{
    misc::{AttrSpec, EffectSpec},
    rd::{REffect, REffectId},
    svc::calc::RawModId,
    ud::UItemId,
    util::RMapRSet,
};

/// Intended to hold data about buffs defined using on-item attributes.
#[derive(Clone)]
pub(in crate::svc::calc) struct BuffRegister {
    effect_rids: RMapRSet<UItemId, REffectId>,
    rmods: RMapRSet<AttrSpec, RawModId>,
    projectees: RMapRSet<EffectSpec, UItemId>,
}
impl BuffRegister {
    pub(in crate::svc::calc) fn new() -> Self {
        Self {
            effect_rids: RMapRSet::new(),
            rmods: RMapRSet::new(),
            projectees: RMapRSet::new(),
        }
    }
    // Effect methods
    pub(in crate::svc::calc) fn get_effects(&self, item_uid: &UItemId) -> impl ExactSizeIterator<Item = REffectId> {
        self.effect_rids.get(item_uid).copied()
    }
    pub(in crate::svc::calc) fn reg_effect(&mut self, item_uid: UItemId, effect: &REffect) {
        if uses_default_attrs(effect) {
            self.effect_rids.add_entry(item_uid, effect.rid);
        }
    }
    pub(in crate::svc::calc) fn unreg_effect(&mut self, item_uid: UItemId, effect: &REffect) {
        if uses_default_attrs(effect) {
            self.effect_rids.remove_entry(item_uid, &effect.rid);
        }
    }
    // Projection methods
    pub(in crate::svc::calc) fn iter_projectees(&self, espec: &EffectSpec) -> impl ExactSizeIterator<Item = UItemId> {
        self.projectees.get(espec).copied()
    }
    pub(in crate::svc::calc) fn reg_projection(&mut self, espec: EffectSpec, effect: &REffect, projectee_uid: UItemId) {
        if uses_default_attrs(effect) {
            self.projectees.add_entry(espec, projectee_uid);
        }
    }
    pub(in crate::svc::calc) fn unreg_projection(
        &mut self,
        espec: EffectSpec,
        effect: &REffect,
        projectee_uid: UItemId,
    ) {
        if uses_default_attrs(effect) {
            self.projectees.remove_entry(espec, &projectee_uid);
        }
    }
    // Modifier methods
    pub(in crate::svc::calc) fn extract_dependent_mods(
        &mut self,
        aspec: &AttrSpec,
    ) -> impl ExactSizeIterator<Item = RawModId> + use<> {
        self.rmods.remove_key(aspec)
    }
    pub(in crate::svc::calc) fn reg_dependent_mod(&mut self, aspec: AttrSpec, rmod_id: RawModId) {
        self.rmods.add_entry(aspec, rmod_id)
    }
    pub(in crate::svc::calc) fn unreg_dependent_mod(&mut self, aspec: AttrSpec, rmod_id: &RawModId) {
        self.rmods.remove_entry(aspec, rmod_id);
    }
}

fn uses_default_attrs(effect: &REffect) -> bool {
    match &effect.buff {
        Some(effect_buff) => effect_buff.attr_merge.is_some(),
        _ => false,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, svc::calc::registers::RawModStorage, ud::UData};

    impl BuffRegister {
        pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData, rmods: &RawModStorage) -> DebugResult {
            let Self {
                effect_rids,
                rmods: aspec_rmods,
                projectees,
            } = self;
            for (&item_uid, effect_rids) in effect_rids.iter() {
                item_uid.consistency_check(u_data, true)?;
                for &effect_rid in effect_rids {
                    effect_rid.consistency_check(u_data)?;
                }
            }
            for (aspec, rmod_ids) in aspec_rmods.iter() {
                aspec.consistency_check(u_data, true)?;
                for &rmod_id in rmod_ids {
                    rmod_id.consistency_check(u_data, rmods)?;
                }
            }
            for (espec, projectee_uids) in projectees.iter() {
                espec.consistency_check(u_data, true)?;
                for &projectee_uid in projectee_uids {
                    // Projectees don't have to be loaded for an entry to be added here
                    projectee_uid.consistency_check(u_data, false)?;
                }
            }
            Ok(())
        }
    }
}
