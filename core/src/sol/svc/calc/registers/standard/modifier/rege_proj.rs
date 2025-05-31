use itertools::Itertools;

use crate::sol::{
    ItemKey,
    svc::{
        EffectSpec,
        calc::{CtxModifier, ModifierKind, RawModifier, registers::StandardRegister},
    },
    uad::item::UadItem,
};

impl StandardRegister {
    pub(in crate::sol::svc::calc) fn reg_proj_mod(&mut self, raw_modifier: RawModifier) {
        // Register projectable modifier.
        self.rmods_all.add_entry(raw_modifier.affector_espec, raw_modifier);
        self.rmods_proj.add_entry(raw_modifier.affector_espec, raw_modifier);
    }
    pub(in crate::sol::svc::calc) fn unreg_proj_mod(&mut self, raw_modifier: &RawModifier) {
        // Unregister projectable modifiers. The rmods_all container should be emptied by the
        // caller, so we do not need to take care about it here.
        self.rmods_proj.remove_entry(&raw_modifier.affector_espec, raw_modifier);
    }
    pub(in crate::sol::svc::calc) fn project_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Vec<CtxModifier> {
        // Register projection and get appropriate context modifiers.
        let raw_modifiers = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut ctx_modifiers = Vec::with_capacity(raw_modifiers.len());
        for raw_modifier in raw_modifiers.iter() {
            if let Some(ctx_modifier) = match raw_modifier.kind {
                ModifierKind::System => self.proj_system_mod(*raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Targeted => self.proj_target_mod(*raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Buff => self.proj_buff_mod(*raw_modifier, projectee_item_key, projectee_item),
                _ => None,
            } {
                ctx_modifiers.push(ctx_modifier);
            }
        }
        ctx_modifiers
    }
    pub(in crate::sol::svc::calc) fn query_projected_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Vec<CtxModifier> {
        // Get context modifiers for projection.
        let raw_modifiers = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut ctx_modifiers = Vec::with_capacity(raw_modifiers.len());
        for raw_modifier in raw_modifiers.iter() {
            if let Some(ctx_modifier) = match raw_modifier.kind {
                ModifierKind::System => self.query_system_mod(*raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Targeted => self.query_target_mod(*raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Buff => self.query_buff_mod(*raw_modifier, projectee_item_key, projectee_item),
                _ => None,
            } {
                ctx_modifiers.push(ctx_modifier);
            }
        }
        ctx_modifiers
    }
    pub(in crate::sol::svc::calc) fn unproject_effect(
        &mut self,
        projector_espec: &EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) -> Vec<CtxModifier> {
        // Unregister projection and get appropriate context modifiers.
        let raw_modifiers = self.rmods_proj.get(projector_espec).copied().collect_vec();
        let mut ctx_modifiers = Vec::with_capacity(raw_modifiers.len());
        for raw_modifier in raw_modifiers.iter() {
            if let Some(ctx_modifier) = match raw_modifier.kind {
                ModifierKind::System => self.unproj_system_mod(*raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Targeted => self.unproj_target_mod(*raw_modifier, projectee_item_key, projectee_item),
                ModifierKind::Buff => self.unproj_buff_mod(*raw_modifier, projectee_item_key, projectee_item),
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
                match raw_modifier.kind {
                    ModifierKind::System => (),
                    ModifierKind::Targeted => (),
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
                match raw_modifier.kind {
                    ModifierKind::System => (),
                    ModifierKind::Targeted => (),
                    ModifierKind::Buff => {
                        self.unreg_loc_root_for_proj_buff(raw_modifier, projectee_item_key, projectee_item)
                    }
                    _ => (),
                }
            }
        }
    }
}
