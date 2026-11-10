use super::{
    rege_proj_buff::{process_affectee_for_proj_buff, process_buff_mod_projection, query_buff_mod},
    rege_proj_system::process_system_mod_projection,
    rege_proj_target::{process_affectee_for_proj_target, process_target_mod_projection, query_target_mod},
};
use crate::{
    misc::EffectSpec,
    svc::calc::{
        CtxModifier, ModifierKind, RawModId, RawModTemplates, RawModifier,
        registers::{
            RawModStorage, RegOp, StandardRegister, process_entry,
            standard::data::{StandardRegisterCtxMods, StandardRegisterRawProjStatus},
        },
    },
    ud::{UItem, UItemId},
};

impl StandardRegister {
    pub(in crate::svc::calc) fn process_proj_mod<const OP: RegOp>(&mut self, rmod_id: RawModId, rmod: &RawModifier) {
        process_entry::<OP, _, _>(&mut self.rmods_proj, rmod.affector_espec, rmod_id);
    }
    pub(in crate::svc::calc) fn process_effect_projection<const OP: RegOp>(
        &mut self,
        reuse_cmods: &mut Vec<CtxModifier>,
        templates: &RawModTemplates,
        rmods: &RawModStorage,
        projector_espec: EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Process projection and get appropriate context modifiers.
        let rmod_ids = self.rmods_proj.get(&projector_espec);
        for &rmod_id in rmod_ids {
            let rmod = rmods.get_rmod(templates, rmod_id);
            if let Some(cmod) = process_mod_projection::<OP>(
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
    pub(in crate::svc::calc) fn process_mod_projection<const OP: RegOp>(
        &mut self,
        rmod_id: RawModId,
        rmod: RawModifier,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) -> Option<CtxModifier> {
        process_mod_projection::<OP>(
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
        templates: &RawModTemplates,
        rmods: &RawModStorage,
        projector_espec: EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Get context modifiers for projection.
        let rmod_ids = self.rmods_proj.get(&projector_espec);
        for &rmod_id in rmod_ids {
            let rmod = rmods.get_rmod(templates, rmod_id);
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
    pub(in crate::svc::calc::registers::standard) fn process_affectee_for_proj<const OP: RegOp>(
        &mut self,
        templates: &RawModTemplates,
        rmods: &RawModStorage,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Loading a projectee can activate modifiers projected onto it, unloading deactivates them
        let proj_status = &mut self.rmods_proj_status;
        let (from, to) = match OP {
            RegOp::Reg => (&mut proj_status.inactive, &mut proj_status.active),
            RegOp::Unreg => (&mut proj_status.active, &mut proj_status.inactive),
        };
        from.buffer_if(projectee_uid, |&rmod_id| {
            let rmod = rmods.get_rmod(templates, rmod_id);
            match rmod.kind {
                ModifierKind::Targeted => process_affectee_for_proj_target::<OP>(
                    &mut self.cmods,
                    rmod_id,
                    &rmod,
                    projectee_uid,
                    projectee_item,
                ),
                ModifierKind::Buff => {
                    process_affectee_for_proj_buff::<OP>(&mut self.cmods, rmod_id, &rmod, projectee_uid, projectee_item)
                }
                _ => false,
            }
        });
        to.extend_entries(projectee_uid, from.drain_buffer());
    }
}

fn process_mod_projection<const OP: RegOp>(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod_id: RawModId,
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.kind {
        ModifierKind::System => process_system_mod_projection::<OP>(reg_cmods, rmod_id, rmod, projectee_item),
        ModifierKind::Targeted => process_target_mod_projection::<OP>(
            reg_proj_status,
            reg_cmods,
            rmod_id,
            rmod,
            projectee_uid,
            projectee_item,
        ),
        ModifierKind::Buff => {
            process_buff_mod_projection::<OP>(reg_proj_status, reg_cmods, rmod_id, rmod, projectee_uid, projectee_item)
        }
        _ => None,
    }
}
