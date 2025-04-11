use itertools::Itertools;

use crate::{
    ad,
    sol::{
        ItemKey,
        svc::calc::{CtxModifier, ModifierKind, RawModifier, registers::StandardRegister},
        uad::item::Item,
    },
};

impl StandardRegister {
    pub(in crate::sol::svc::calc) fn reg_proj_mod(&mut self, raw_modifier: RawModifier) {
        self.rmods_proj
            .add_entry((raw_modifier.affector_item_key, raw_modifier.a_effect_id), raw_modifier)
    }
    pub(in crate::sol::svc::calc) fn project_effect(
        &mut self,
        projector_item_key: ItemKey,
        a_effect_id: ad::AEffectId,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) -> Vec<CtxModifier> {
        let raw_modifiers = self
            .rmods_proj
            .get(&(projector_item_key, a_effect_id))
            .copied()
            .collect_vec();
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
        projector_item_key: ItemKey,
        a_effect_id: ad::AEffectId,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) -> Vec<CtxModifier> {
        let raw_modifiers = self
            .rmods_proj
            .get(&(projector_item_key, a_effect_id))
            .copied()
            .collect_vec();
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
        projector_item_key: ItemKey,
        a_effect_id: ad::AEffectId,
        projectee_item_key: ItemKey,
        projectee_item: &Item,
    ) -> Vec<CtxModifier> {
        let raw_modifiers = self
            .rmods_proj
            .get(&(projector_item_key, a_effect_id))
            .copied()
            .collect_vec();
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
}
