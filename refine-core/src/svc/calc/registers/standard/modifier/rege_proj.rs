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
    svc::calc::{
        CtxModifier, ModifierKind, RawModId, RawModifier,
        registers::{
            RawModStorage, StandardRegister,
            standard::data::{StandardRegisterCtxMods, StandardRegisterRawProjStatus},
        },
    },
    ud::{UItem, UItemId},
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_proj_mod(&mut self, rmod_id: RawModId, rmod: &RawModifier) {
        self.rmods_proj.add_entry(rmod.affector_espec, rmod_id);
    }
    pub(in crate::svc::calc) fn unreg_proj_mod(&mut self, rmod_id: RawModId, rmod: &RawModifier) {
        self.rmods_proj.remove_entry(rmod.affector_espec, &rmod_id);
    }
    pub(in crate::svc::calc) fn project_effect(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        rmods: &RawModStorage,
        projector_espec: &EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Register projection and get appropriate context modifiers.
        let rmod_ids = self.rmods_proj.get(projector_espec);
        reuse_cmods.clear();
        for &rmod_id in rmod_ids {
            let rmod = *rmods.get_rmod(rmod_id);
            if let Some(cmod) = proj_mod(
                &mut self.rmods_proj_status,
                &mut self.cmods,
                rmod_id,
                rmod,
                projectee_uid,
                projectee_item,
            ) {
                reuse_cmods.push(cmod);
            }
        }
    }
    pub(in crate::svc::calc) fn project_mod(
        &mut self,
        rmod_id: RawModId,
        rmod: RawModifier,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        proj_mod(
            &mut self.rmods_proj_status,
            &mut self.cmods,
            rmod_id,
            rmod,
            projectee_uid,
            projectee_item,
        )
    }
    pub(in crate::svc::calc) fn unproject_effect(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        rmods: &RawModStorage,
        projector_espec: &EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Unregister projection and get appropriate context modifiers.
        let rmod_ids = self.rmods_proj.get(projector_espec);
        reuse_cmods.clear();
        for &rmod_id in rmod_ids {
            let rmod = *rmods.get_rmod(rmod_id);
            if let Some(cmod) = unproj_mod(
                &mut self.rmods_proj_status,
                &mut self.cmods,
                rmod_id,
                rmod,
                projectee_uid,
                projectee_item,
            ) {
                reuse_cmods.push(cmod);
            }
        }
    }
    pub(in crate::svc::calc) fn unproject_mod(
        &mut self,
        rmod_id: RawModId,
        rmod: RawModifier,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        unproj_mod(
            &mut self.rmods_proj_status,
            &mut self.cmods,
            rmod_id,
            rmod,
            projectee_uid,
            projectee_item,
        )
    }
    pub(in crate::svc::calc) fn query_projected_effect(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        rmods: &RawModStorage,
        projector_espec: &EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Get context modifiers for projection.
        let rmod_ids = self.rmods_proj.get(projector_espec);
        reuse_cmods.clear();
        for &rmod_id in rmod_ids {
            let rmod = *rmods.get_rmod(rmod_id);
            if let Some(cmod) = match rmod.kind {
                // System modifiers are not requested, since their application does not depend on
                // projection attributes (like range)
                ModifierKind::Targeted => query_target_mod(rmod, projectee_uid, projectee_item),
                ModifierKind::Buff => query_buff_mod(rmod, projectee_uid, projectee_item),
                _ => None,
            } {
                reuse_cmods.push(cmod);
            }
        }
    }
    pub(in crate::svc::calc::registers::standard) fn load_affectee_for_proj(
        &mut self,
        rmods: &RawModStorage,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        self.rmods_proj_status.inactive.buffer_if(projectee_uid, |&rmod_id| {
            let rmod = rmods.get_rmod(rmod_id);
            match rmod.kind {
                ModifierKind::Targeted => {
                    load_affectee_for_proj_target(&mut self.cmods, rmod_id, rmod, projectee_uid, projectee_item)
                }
                ModifierKind::Buff => {
                    load_affectee_for_proj_buff(&mut self.cmods, rmod_id, rmod, projectee_uid, projectee_item)
                }
                _ => false,
            }
        });
        self.rmods_proj_status
            .active
            .extend_entries(projectee_uid, self.rmods_proj_status.inactive.drain_buffer());
    }
    pub(in crate::svc::calc::registers::standard) fn unload_affectee_for_proj(
        &mut self,
        rmods: &RawModStorage,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        self.rmods_proj_status.active.buffer_if(projectee_uid, |&rmod_id| {
            let rmod = rmods.get_rmod(rmod_id);
            match rmod.kind {
                ModifierKind::Targeted => {
                    unload_affectee_for_proj_target(&mut self.cmods, rmod_id, rmod, projectee_uid, projectee_item)
                }
                ModifierKind::Buff => {
                    unload_affectee_for_proj_buff(&mut self.cmods, rmod_id, rmod, projectee_uid, projectee_item)
                }
                _ => false,
            }
        });
        self.rmods_proj_status
            .inactive
            .extend_entries(projectee_uid, self.rmods_proj_status.active.drain_buffer());
    }
}

fn proj_mod(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod_id: RawModId,
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.kind {
        ModifierKind::System => proj_system_mod(reg_cmods, rmod_id, rmod, projectee_item),
        ModifierKind::Targeted => {
            proj_target_mod(reg_proj_status, reg_cmods, rmod_id, rmod, projectee_uid, projectee_item)
        }
        ModifierKind::Buff => proj_buff_mod(reg_proj_status, reg_cmods, rmod_id, rmod, projectee_uid, projectee_item),
        _ => None,
    }
}

fn unproj_mod(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod_id: RawModId,
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.kind {
        ModifierKind::System => unproj_system_mod(reg_cmods, rmod_id, rmod, projectee_item),
        ModifierKind::Targeted => {
            unproj_target_mod(reg_proj_status, reg_cmods, rmod_id, rmod, projectee_uid, projectee_item)
        }
        ModifierKind::Buff => unproj_buff_mod(reg_proj_status, reg_cmods, rmod_id, rmod, projectee_uid, projectee_item),
        _ => None,
    }
}
