use itertools::Itertools;

use crate::{
    defs::{EEffectId, SolItemId},
    sol::{
        item::SolItem,
        svc::svce_calc::{registers::SolStandardRegister, SolCtxModifier, SolModifierKind, SolRawModifier},
    },
};

impl SolStandardRegister {
    pub(in crate::sol::svc::svce_calc) fn reg_proj_mod(&mut self, raw_modifier: SolRawModifier) {
        self.rmods_proj
            .add_entry((raw_modifier.affector_item_id, raw_modifier.effect_id), raw_modifier);
    }
    pub(in crate::sol::svc::svce_calc) fn project_effect(
        &mut self,
        projector_item_id: &SolItemId,
        effect_id: &EEffectId,
        projectee_item: &SolItem,
    ) -> Vec<SolCtxModifier> {
        let raw_modifiers = self
            .rmods_proj
            .get(&(*projector_item_id, *effect_id))
            .map(|v| *v)
            .collect_vec();
        let mut ctx_modifiers = Vec::with_capacity(raw_modifiers.len());
        for raw_modifier in raw_modifiers.iter() {
            if let Some(ctx_modifier) = match raw_modifier.kind {
                SolModifierKind::System => self.proj_system_mod(*raw_modifier, projectee_item),
                SolModifierKind::Targeted => self.proj_target_mod(*raw_modifier, projectee_item),
                SolModifierKind::Buff => self.proj_buff_mod(*raw_modifier, projectee_item),
                _ => None,
            } {
                ctx_modifiers.push(ctx_modifier);
            }
        }
        ctx_modifiers
    }
    pub(in crate::sol::svc::svce_calc) fn unproject_effect(
        &mut self,
        projector_item_id: &SolItemId,
        effect_id: &EEffectId,
        projectee_item: &SolItem,
    ) -> Vec<SolCtxModifier> {
        let raw_modifiers = self
            .rmods_proj
            .get(&(*projector_item_id, *effect_id))
            .map(|v| *v)
            .collect_vec();
        let mut ctx_modifiers = Vec::with_capacity(raw_modifiers.len());
        for raw_modifier in raw_modifiers.iter() {
            if let Some(ctx_modifier) = match raw_modifier.kind {
                SolModifierKind::System => self.unproj_system_mod(*raw_modifier, projectee_item),
                SolModifierKind::Targeted => self.unproj_target_mod(*raw_modifier, projectee_item),
                SolModifierKind::Buff => self.unproj_buff_mod(*raw_modifier, projectee_item),
                _ => None,
            } {
                ctx_modifiers.push(ctx_modifier);
            }
        }
        ctx_modifiers
    }
}
