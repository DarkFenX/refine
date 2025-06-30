use itertools::Itertools;

use crate::{
    def::ItemKey,
    misc::EffectSpec,
    svc::calc::{CtxModifier, ModifierKind, RawModifier, registers::StandardRegister},
    uad::UadItem,
};

impl StandardRegister {
    pub(in crate::svc::calc) fn reg_proj_mod(&mut self, raw_modifier: RawModifier) {
        // Register projectable modifier.
        self.rmods_all.add_entry(raw_modifier.affector_espec, raw_modifier);
        self.rmods_proj.add_entry(raw_modifier.affector_espec, raw_modifier);
    }
    pub(in crate::svc::calc) fn unreg_proj_mod(&mut self, raw_modifier: &RawModifier) {
        // Unregister projectable modifiers. The rmods_all container should be emptied by the
        // caller, so we do not need to take care about it here.
        self.rmods_proj.remove_entry(&raw_modifier.affector_espec, raw_modifier);
    }
    pub(in crate::svc::calc) fn project_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Vec<CtxModifier> {
        // Register projection and get appropriate context modifiers.
        let raw_modifiers = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut ctx_modifiers = Vec::with_capacity(raw_modifiers.len());
        for raw_modifier in raw_modifiers.into_iter() {
            // Validate raw modifier. If it is valid and target passes all checks, create and store
            // appropriate context modifiers, put raw modifier into active projected modifier
            // storage, and add context modifier to container. If it valid and target doesn't pass
            // all checks, put raw modifier into inactive projected modifier storage.
            if let Some(ctx_modifier) = match raw_modifier.kind {
                ModifierKind::System => self.proj_system_mod(raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Targeted => self.proj_target_mod(raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Buff => self.proj_buff_mod(raw_modifier, projectee_item_key, projectee_item),
                _ => None,
            } {
                ctx_modifiers.push(ctx_modifier);
            }
        }
        ctx_modifiers
    }
    pub(in crate::svc::calc) fn query_projected_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Vec<CtxModifier> {
        // Get context modifiers for projection.
        let raw_modifiers = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut ctx_modifiers = Vec::with_capacity(raw_modifiers.len());
        for raw_modifier in raw_modifiers.into_iter() {
            // Validate raw modifier and its target, return context modifier if both pass checks.
            if let Some(ctx_modifier) = match raw_modifier.kind {
                ModifierKind::System => self.query_system_mod(raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Targeted => self.query_target_mod(raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Buff => self.query_buff_mod(raw_modifier, projectee_item_key, projectee_item),
                _ => None,
            } {
                ctx_modifiers.push(ctx_modifier);
            }
        }
        ctx_modifiers
    }
    pub(in crate::svc::calc) fn unproject_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Vec<CtxModifier> {
        // Unregister projection and get appropriate context modifiers.
        let raw_modifiers = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut ctx_modifiers = Vec::with_capacity(raw_modifiers.len());
        for raw_modifier in raw_modifiers.into_iter() {
            // Validate raw modifier. If it is valid and target passes all checks, remove
            // appropriate context modifiers, remove raw modifier from active projected modifier
            // storage, and add context modifier to container. If it is valid and target doesn't
            // pass all checks, remove raw modifier from inactive projected modifier storage.
            if let Some(ctx_modifier) = match raw_modifier.kind {
                ModifierKind::System => self.unproj_system_mod(raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Targeted => self.unproj_target_mod(raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Buff => self.unproj_buff_mod(raw_modifier, projectee_item_key, projectee_item),
                _ => None,
            } {
                ctx_modifiers.push(ctx_modifier);
            }
        }
        ctx_modifiers
    }
    pub(super) fn reg_loc_root_for_proj(&mut self, projectee_item_key: ItemKey, projectee_item: &UadItem) {
        // Do necessary changes to projected modifiers after adding location root.
        if let Some(raw_modifiers) = self.rmods_proj_inactive.remove_key(&projectee_item_key) {
            for raw_modifier in raw_modifiers {
                // Store appropriate context modifiers, and put raw modifier into either active or
                // inactive storage, depending on projectee. I.e. the same thing done when adding
                // projected modifier. Emptying of inactive projected modifier storage has already
                // been done before, so modifier kind-specific methods are not handling that.
                match raw_modifier.kind {
                    ModifierKind::System => {
                        self.reg_loc_root_for_proj_system(raw_modifier, projectee_item_key, projectee_item)
                    }
                    ModifierKind::Targeted => {
                        self.reg_loc_root_for_proj_target(raw_modifier, projectee_item_key, projectee_item)
                    }
                    ModifierKind::Buff => {
                        self.reg_loc_root_for_proj_buff(raw_modifier, projectee_item_key, projectee_item)
                    }
                    _ => (),
                }
            }
        }
    }
    pub(super) fn unreg_loc_root_for_proj(&mut self, projectee_item_key: ItemKey, projectee_item: &UadItem) {
        // Do necessary changes to projected modifiers before removing location root.
        if let Some(raw_modifiers) = self.rmods_proj_active.remove_key(&projectee_item_key) {
            for raw_modifier in raw_modifiers {
                // Remove context modifiers for passed raw modifier + projection target, and add raw
                // modifier to inactive storage. Emptying of inactive projected modifier storage has
                // already been done before, so modifier kind-specific methods are not handling
                // that.
                match raw_modifier.kind {
                    ModifierKind::System => {
                        self.unreg_loc_root_for_proj_system(raw_modifier, projectee_item_key, projectee_item)
                    }
                    ModifierKind::Targeted => {
                        self.unreg_loc_root_for_proj_target(raw_modifier, projectee_item_key, projectee_item)
                    }
                    ModifierKind::Buff => {
                        self.unreg_loc_root_for_proj_buff(raw_modifier, projectee_item_key, projectee_item)
                    }
                    _ => (),
                }
            }
        }
    }
    // Utility methods for use in more specific modules
    pub(super) fn reg_inactive_proj_rmod(
        &mut self,
        raw_modifier: RawModifier,
        projectee_item_key: ItemKey,
        register: bool,
    ) -> Option<CtxModifier> {
        if register {
            self.rmods_proj_inactive.add_entry(projectee_item_key, raw_modifier);
        }
        None
    }
    pub(super) fn unreg_inactive_proj_rmod(
        &mut self,
        raw_modifier: &RawModifier,
        projectee_item_key: &ItemKey,
    ) -> Option<CtxModifier> {
        self.rmods_proj_inactive.remove_entry(projectee_item_key, raw_modifier);
        None
    }
}
