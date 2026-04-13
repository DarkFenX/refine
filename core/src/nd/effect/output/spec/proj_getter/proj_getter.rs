use super::mult::{
    application::{get_bomb_application_mult, get_standard_missile_application_mult},
    composite::{
        get_aoe_burst_proj_mult, get_aoe_dd_dmg_proj_mult, get_aoe_dd_side_neut_proj_mult, get_disintegrator_proj_mult,
        get_ftr_abil_attack_m_proj_mult, get_ftr_abil_missiles_proj_mult, get_neut_proj_mult, get_turret_proj_mult,
        get_vorton_proj_mult,
    },
    range::{
        get_aoe_burst_range_mult, get_aoe_dd_range_mult, get_bomb_range_mult, get_fof_missile_range_mult,
        get_full_restricted_range_mult, get_missile_range_mult, get_simple_c2s_range_mult, get_simple_s2s_range_mult,
    },
};
use crate::{
    ad::{AAttrId, AEffect},
    num::PValue,
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemId, UProjData},
};

#[derive(Copy, Clone)]
pub(crate) enum NEffectProjGetter {
    Null,
    GenericRangeSimpleCts,
    GenericRangeSimpleSts,
    GenericRangeFullStsRestricted,
    Turret,
    Disintegrator,
    Vorton,
    MissileRange,
    MissileRangeFof,
    MissileApplication,
    BombRange,
    BombApplication,
    Neut,
    AoeDd,
    AoeDdRange,
    AoeDdWarmupNeut,
    AoeBurst,
    AoeBurstRange,
    // Variants specific to a single effect
    MissileLaunchingApplication,
    FtrAbilAttackM,
    FtrAbilMissiles,
}
impl NEffectProjGetter {
    pub(crate) fn get_mult(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_uid: UItemId,
        effect: &REffect,
        projectee_uid: UItemId,
        proj_data: UProjData,
    ) -> PValue {
        match self {
            Self::Null => PValue::ZERO,
            Self::GenericRangeSimpleCts => get_simple_c2s_range_mult(ctx, calc, projector_uid, effect, proj_data),
            Self::GenericRangeSimpleSts => get_simple_s2s_range_mult(ctx, calc, projector_uid, effect, proj_data),
            Self::GenericRangeFullStsRestricted => {
                get_full_restricted_range_mult(ctx, calc, projector_uid, effect, proj_data)
            }
            Self::Turret => get_turret_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::Disintegrator => {
                get_disintegrator_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data)
            }
            Self::Vorton => get_vorton_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::MissileRange => get_missile_range_mult(ctx, calc, projector_uid, proj_data),
            Self::MissileRangeFof => get_fof_missile_range_mult(ctx, calc, projector_uid, proj_data),
            Self::MissileApplication => {
                get_standard_missile_application_mult(ctx, calc, projector_uid, projectee_uid, proj_data)
            }
            Self::BombRange => get_bomb_range_mult(ctx, calc, projector_uid, proj_data),
            Self::BombApplication => get_bomb_application_mult(ctx, calc, projector_uid, projectee_uid),
            Self::Neut => get_neut_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::AoeDd => get_aoe_dd_dmg_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
            Self::AoeDdRange => get_aoe_dd_range_mult(ctx, calc, projector_uid, proj_data),
            Self::AoeDdWarmupNeut => get_aoe_dd_side_neut_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
            Self::AoeBurst => get_aoe_burst_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::AoeBurstRange => get_aoe_burst_range_mult(ctx, calc, projector_uid, effect, proj_data),
            // Variants specific to a single effect
            Self::MissileLaunchingApplication => {
                let u_item = ctx.u_data.items.get(projector_uid);
                match u_item.is_guided_bomb() {
                    true => get_bomb_application_mult(ctx, calc, projector_uid, projectee_uid),
                    false => get_standard_missile_application_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
                }
            }
            Self::FtrAbilAttackM => {
                get_ftr_abil_attack_m_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data)
            }
            Self::FtrAbilMissiles => {
                get_ftr_abil_missiles_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data)
            }
        }
    }
    // Returns attributes which can affect modifier application strength
    pub(crate) fn get_modifier_attr_aids(&self, a_effect: &AEffect) -> [Option<AAttrId>; 2] {
        // Only variants actually used to project modifiers are filled
        match self {
            Self::Null => [None, None],
            Self::GenericRangeSimpleCts => [a_effect.range_attr_id, None],
            Self::GenericRangeSimpleSts => [a_effect.range_attr_id, None],
            Self::GenericRangeFullStsRestricted => [a_effect.range_attr_id, a_effect.falloff_attr_id],
            Self::Turret => [None, None],
            Self::Disintegrator => [None, None],
            Self::Vorton => [None, None],
            Self::MissileRange => [None, None],
            Self::MissileRangeFof => [None, None],
            Self::MissileApplication => [None, None],
            Self::BombRange => [None, None],
            Self::BombApplication => [None, None],
            Self::Neut => [None, None],
            Self::AoeDd => [None, None],
            Self::AoeDdRange => [Some(AAttrId::MAX_RANGE), None],
            Self::AoeDdWarmupNeut => [None, None],
            Self::AoeBurst => [None, None],
            Self::AoeBurstRange => [Some(AAttrId::MAX_RANGE), Some(AAttrId::DOOMSDAY_AOE_RANGE)],
            // Variants specific to a single effect
            Self::MissileLaunchingApplication => [None, None],
            Self::FtrAbilAttackM => [None, None],
            Self::FtrAbilMissiles => [None, None],
        }
    }
}
