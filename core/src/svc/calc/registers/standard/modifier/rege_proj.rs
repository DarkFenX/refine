use itertools::Itertools;

use super::{
    rege_proj_buff::{load_affectee_for_proj_buff, unload_affectee_for_proj_buff},
    rege_proj_target::{load_affectee_for_proj_target, unload_affectee_for_proj_target},
};
use crate::{
    misc::EffectSpec,
    svc::calc::{CtxModifier, ModifierKind, RawModifier, registers::StandardRegister},
    ud::{UItem, UItemKey},
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
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Vec<CtxModifier> {
        // Register projection and get appropriate context modifiers.
        let rmods = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut cmods = Vec::with_capacity(rmods.len());
        for rmod in rmods.into_iter() {
            // Validate raw modifier. If it is valid and target passes all checks, create and store
            // appropriate context modifiers, put raw modifier into active projected modifier
            // storage, and add context modifier to container. If it valid and target doesn't pass
            // all checks, put raw modifier into inactive projected modifier storage.
            if let Some(cmod) = match rmod.kind {
                ModifierKind::System => self.proj_system_mod(rmod, projectee_item),
                ModifierKind::Targeted => self.proj_target_mod(rmod, projectee_key, projectee_item),
                ModifierKind::Buff => self.proj_buff_mod(rmod, projectee_key, projectee_item),
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
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Vec<CtxModifier> {
        // Get context modifiers for projection.
        let rmods = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut cmods = Vec::with_capacity(rmods.len());
        for rmod in rmods.into_iter() {
            // Validate raw modifier and its target, return context modifier if both pass checks.
            if let Some(cmod) = match rmod.kind {
                ModifierKind::Targeted => self.query_target_mod(rmod, projectee_key, projectee_item),
                ModifierKind::Buff => self.query_buff_mod(rmod, projectee_key, projectee_item),
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
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) -> Vec<CtxModifier> {
        // Unregister projection and get appropriate context modifiers.
        let rmods = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut cmods = Vec::with_capacity(rmods.len());
        for rmod in rmods.into_iter() {
            // Validate raw modifier. If it is valid and target passes all checks, remove
            // appropriate context modifiers, remove raw modifier from active projected modifier
            // storage, and add context modifier to container. If it is valid and target doesn't
            // pass all checks, remove raw modifier from inactive projected modifier storage.
            if let Some(cmod) = match rmod.kind {
                ModifierKind::System => self.unproj_system_mod(rmod, projectee_item),
                ModifierKind::Targeted => self.unproj_target_mod(rmod, projectee_key, projectee_item),
                ModifierKind::Buff => self.unproj_buff_mod(rmod, projectee_key, projectee_item),
                _ => None,
            } {
                cmods.push(cmod);
            }
        }
        cmods
    }
    pub(in crate::svc::calc::registers::standard) fn load_affectee_for_proj(
        &mut self,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        self.rmods_proj_inactive.buffer_if(projectee_key, |r| match r.kind {
            ModifierKind::Targeted => load_affectee_for_proj_target(&mut self.cmods, r, projectee_key, projectee_item),
            ModifierKind::Buff => load_affectee_for_proj_buff(&mut self.cmods, r, projectee_key, projectee_item),
            _ => false,
        });
        self.rmods_proj_active
            .extend_entries(projectee_key, self.rmods_proj_inactive.drain_buffer());
    }
    pub(in crate::svc::calc::registers::standard) fn unload_affectee_for_proj(
        &mut self,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        self.rmods_proj_active.buffer_if(projectee_key, |r| match r.kind {
            ModifierKind::Targeted => {
                unload_affectee_for_proj_target(&mut self.cmods, r, projectee_key, projectee_item)
            }
            ModifierKind::Buff => unload_affectee_for_proj_buff(&mut self.cmods, r, projectee_key, projectee_item),
            _ => false,
        });
        self.rmods_proj_inactive
            .extend_entries(projectee_key, self.rmods_proj_active.drain_buffer());
    }
}
