use super::{
    rege_proj_buff::{
        load_affectee_for_proj_buff, proj_buff_mod, query_buff_mod, unload_affectee_for_proj_buff, unproj_buff_mod,
    },
    rege_proj_system::{proj_system_mod, unproj_system_mod},
    rege_proj_target::{
        load_affectee_for_proj_target, proj_target_mod, query_target_mod, unload_affectee_for_proj_target,
        unproj_target_mod,
    },
};
use crate::{
    misc::EffectSpec,
    svc::calc::{CtxModifier, ModifierKind, RawModifier, registers::StandardRegister},
    ud::{UItem, UItemId},
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_proj_mod(&mut self, rmod: RawModifier) {
        // Register projectable modifier.
        self.rmods_all.add_entry(rmod.affector_espec, rmod);
        self.rmods_proj.add_entry(rmod.affector_espec, rmod);
    }
    pub(in crate::svc::calc) fn unreg_proj_mod(&mut self, rmod: &RawModifier) {
        // Unregister projectable modifiers. The rmods_all container should be emptied by the
        // caller, so we do not need to take care about it here.
        self.rmods_proj.remove_entry(rmod.affector_espec, rmod);
    }
    pub(in crate::svc::calc) fn project_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) -> Vec<CtxModifier> {
        // Register projection and get appropriate context modifiers.
        let rmods = self.rmods_proj.get(projector_espec);
        let mut cmods = Vec::with_capacity(rmods.len());
        for &rmod in rmods.into_iter() {
            if let Some(cmod) = match rmod.kind {
                ModifierKind::System => proj_system_mod(&mut self.cmods, rmod, projectee_item),
                ModifierKind::Targeted => proj_target_mod(
                    &mut self.rmods_proj_status,
                    &mut self.cmods,
                    rmod,
                    projectee_uid,
                    projectee_item,
                ),
                ModifierKind::Buff => proj_buff_mod(
                    &mut self.rmods_proj_status,
                    &mut self.cmods,
                    rmod,
                    projectee_uid,
                    projectee_item,
                ),
                _ => None,
            } {
                cmods.push(cmod);
            }
        }
        cmods
    }
    pub(in crate::svc::calc) fn unproject_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) -> Vec<CtxModifier> {
        // Unregister projection and get appropriate context modifiers.
        let rmods = self.rmods_proj.get(projector_espec);
        let mut cmods = Vec::with_capacity(rmods.len());
        for &rmod in rmods {
            if let Some(cmod) = match rmod.kind {
                ModifierKind::System => unproj_system_mod(&mut self.cmods, rmod, projectee_item),
                ModifierKind::Targeted => unproj_target_mod(
                    &mut self.rmods_proj_status,
                    &mut self.cmods,
                    rmod,
                    projectee_uid,
                    projectee_item,
                ),
                ModifierKind::Buff => unproj_buff_mod(
                    &mut self.rmods_proj_status,
                    &mut self.cmods,
                    rmod,
                    projectee_uid,
                    projectee_item,
                ),
                _ => None,
            } {
                cmods.push(cmod);
            }
        }
        cmods
    }
    pub(in crate::svc::calc) fn query_projected_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) -> Vec<CtxModifier> {
        // Get context modifiers for projection.
        let rmods = self.rmods_proj.get(projector_espec);
        let mut cmods = Vec::with_capacity(rmods.len());
        for &rmod in rmods {
            if let Some(cmod) = match rmod.kind {
                // System modifiers are not requested, since their application does not depend on
                // projection attributes (like range)
                ModifierKind::Targeted => query_target_mod(rmod, projectee_uid, projectee_item),
                ModifierKind::Buff => query_buff_mod(rmod, projectee_uid, projectee_item),
                _ => None,
            } {
                cmods.push(cmod);
            }
        }
        cmods
    }
    pub(in crate::svc::calc::registers::standard) fn load_affectee_for_proj(
        &mut self,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        self.rmods_proj_status
            .inactive
            .buffer_if(projectee_uid, |r| match r.kind {
                ModifierKind::Targeted => {
                    load_affectee_for_proj_target(&mut self.cmods, r, projectee_uid, projectee_item)
                }
                ModifierKind::Buff => load_affectee_for_proj_buff(&mut self.cmods, r, projectee_uid, projectee_item),
                _ => false,
            });
        self.rmods_proj_status
            .active
            .extend_entries(projectee_uid, self.rmods_proj_status.inactive.drain_buffer());
    }
    pub(in crate::svc::calc::registers::standard) fn unload_affectee_for_proj(
        &mut self,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        self.rmods_proj_status
            .active
            .buffer_if(projectee_uid, |r| match r.kind {
                ModifierKind::Targeted => {
                    unload_affectee_for_proj_target(&mut self.cmods, r, projectee_uid, projectee_item)
                }
                ModifierKind::Buff => unload_affectee_for_proj_buff(&mut self.cmods, r, projectee_uid, projectee_item),
                _ => false,
            });
        self.rmods_proj_status
            .inactive
            .extend_entries(projectee_uid, self.rmods_proj_status.active.drain_buffer());
    }
}
