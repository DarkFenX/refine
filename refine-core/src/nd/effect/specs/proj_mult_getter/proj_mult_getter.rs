use super::mult::{
    application::{get_bomb_application_mult, get_std_missile_application_mult},
    composite::{
        get_aoe_burst_proj_mult, get_aoe_dd_dmg_round_proj_mult, get_aoe_dd_dmg_sharp_proj_mult,
        get_aoe_dd_side_neut_proj_mult, get_disintegrator_proj_mult, get_ftr_abil_attack_m_proj_mult,
        get_ftr_abil_kamikaze_proj_mult, get_ftr_abil_missiles_proj_mult, get_neut_proj_mult, get_turret_proj_mult,
        get_vorton_proj_mult,
    },
    range::{
        get_aoe_burst_range_mult, get_aoe_dd_round_range_mult, get_bomb_range_mult, get_fof_missile_range_mult,
        get_missile_range_mult, get_std_full_restricted_range_mult, get_std_simple_c2s_range_mult,
        get_std_simple_s2s_range_mult,
    },
};
use crate::{
    PValue,
    ad::{AAttrId, AEffect},
    rd::REffect,
    stats::StatCritOptions,
    svc::{Calc, SvcCtx},
    ud::{UItemId, UProjData},
};

#[derive(Copy, Clone)]
pub(crate) enum NEffectProjMultGetter {
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
    AoeDdSharp,
    AoeDdRound,
    AoeDdRoundRange,
    AoeDdWarmupNeut,
    AoeBurst,
    AoeBurstRange,
    // Variants specific to a single effect
    MissileLaunchingApplication,
    FtrAbilAttackM,
    FtrAbilMissiles,
    FtrAbilKamikaze,
}
impl NEffectProjMultGetter {
    pub(crate) fn get_proj_mult(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projector_uid: UItemId,
        effect: &REffect,
        projectee_uid: UItemId,
        proj_data: UProjData,
        crit_options: StatCritOptions,
    ) -> PValue {
        match self {
            Self::Null => PValue::ZERO,
            Self::GenericRangeSimpleCts => get_std_simple_c2s_range_mult(ctx, calc, projector_uid, effect, proj_data),
            Self::GenericRangeSimpleSts => get_std_simple_s2s_range_mult(ctx, calc, projector_uid, effect, proj_data),
            Self::GenericRangeFullStsRestricted => {
                get_std_full_restricted_range_mult(ctx, calc, projector_uid, effect, proj_data)
            }
            Self::Turret => {
                get_turret_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data, crit_options)
            }
            Self::Disintegrator => {
                get_disintegrator_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data, crit_options)
            }
            Self::Vorton => get_vorton_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::MissileRange => get_missile_range_mult(ctx, calc, projector_uid, proj_data),
            Self::MissileRangeFof => get_fof_missile_range_mult(ctx, calc, projector_uid, proj_data),
            Self::MissileApplication => {
                get_std_missile_application_mult(ctx, calc, projector_uid, projectee_uid, proj_data)
            }
            Self::BombRange => get_bomb_range_mult(ctx, calc, projector_uid, proj_data),
            Self::BombApplication => get_bomb_application_mult(ctx, calc, projector_uid, projectee_uid),
            Self::Neut => get_neut_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::AoeDdSharp => get_aoe_dd_dmg_sharp_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
            Self::AoeDdRound => get_aoe_dd_dmg_round_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
            Self::AoeDdRoundRange => get_aoe_dd_round_range_mult(ctx, calc, projector_uid, proj_data),
            Self::AoeDdWarmupNeut => get_aoe_dd_side_neut_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
            Self::AoeBurst => get_aoe_burst_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
            Self::AoeBurstRange => get_aoe_burst_range_mult(ctx, calc, projector_uid, proj_data),
            // Variants specific to a single effect
            Self::MissileLaunchingApplication => {
                let u_item = ctx.u_data.items.get(projector_uid);
                match u_item.is_guided_bomb() {
                    true => get_bomb_application_mult(ctx, calc, projector_uid, projectee_uid),
                    false => get_std_missile_application_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
                }
            }
            Self::FtrAbilAttackM => {
                get_ftr_abil_attack_m_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data)
            }
            Self::FtrAbilMissiles => {
                get_ftr_abil_missiles_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data)
            }
            Self::FtrAbilKamikaze => {
                get_ftr_abil_kamikaze_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data)
            }
        }
    }
    pub(crate) fn get_non_proj_mult(&self, crit_options: StatCritOptions) -> Option<PValue> {
        match self {
            Self::Turret | Self::Disintegrator => match crit_options {
                StatCritOptions::Include => Some(PValue::from_f64_unchecked(1.02)),
                StatCritOptions::Exclude => None,
            },
            _ => None,
        }
    }
    // Returns attributes which can affect modifier application strength
    pub(crate) fn get_proj_modifier_attr_aids(&self, a_effect: &AEffect) -> [Option<AAttrId>; 2] {
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
            Self::AoeDdSharp => [None, None],
            Self::AoeDdRound => [None, None],
            Self::AoeDdRoundRange => [Some(AAttrId::MAX_RANGE), Some(AAttrId::DOOMSDAY_DMG_RADIUS)],
            Self::AoeDdWarmupNeut => [None, None],
            Self::AoeBurst => [None, None],
            Self::AoeBurstRange => [Some(AAttrId::MAX_RANGE), Some(AAttrId::DOOMSDAY_AOE_RANGE)],
            // Variants specific to a single effect
            Self::MissileLaunchingApplication => [None, None],
            Self::FtrAbilAttackM => [None, None],
            Self::FtrAbilMissiles => [None, None],
            Self::FtrAbilKamikaze => [None, None],
        }
    }
}
