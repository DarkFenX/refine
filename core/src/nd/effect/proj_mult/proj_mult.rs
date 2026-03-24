use super::{
    application::{get_bomb_application_mult, get_missile_application_mult, get_missile_or_bomb_application_mult},
    composite::{
        get_aoe_burst_proj_mult, get_aoe_dd_dmg_proj_mult, get_aoe_dd_side_neut_proj_mult, get_disintegrator_proj_mult,
        get_neut_proj_mult, get_turret_proj_mult, get_vorton_proj_mult,
    },
    range::{
        get_aoe_burst_range_mult, get_bomb_range_mult, get_fof_missile_range_mult, get_full_restricted_range_mult,
        get_missile_range_mult, get_simple_s2s_range_mult,
    },
};
use crate::{
    num::PValue,
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemId, UProjData},
};

#[derive(Copy, Clone)]
pub(crate) enum NEffectProjMultGetterX {
    Null,
    RangeSimpleSts,
    RangeFullStsRestricted,
    Turret,
    Disintegrator,
    Vorton,
    MissileRange,
    MissileRangeFof,
    MissileApplication,
    BombRange,
    BombApplication,
    MissileOrBombApplication,
    Neut,
    AoeDdDmg,
    AoeDdSideNeut,
    AoeBurst,
    AoeBurstRange,
}
impl NEffectProjMultGetterX {
    pub(crate) fn get(
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
            Self::RangeSimpleSts => get_simple_s2s_range_mult(ctx, calc, projector_uid, effect, proj_data),
            Self::RangeFullStsRestricted => get_full_restricted_range_mult(ctx, calc, projector_uid, effect, proj_data),
            Self::Turret => get_turret_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::Disintegrator => {
                get_disintegrator_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data)
            }
            Self::Vorton => get_vorton_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::MissileRange => get_missile_range_mult(ctx, calc, projector_uid, proj_data),
            Self::MissileRangeFof => get_fof_missile_range_mult(ctx, calc, projector_uid, proj_data),
            Self::MissileApplication => {
                get_missile_application_mult(ctx, calc, projector_uid, projectee_uid, proj_data)
            }
            Self::BombRange => get_bomb_range_mult(ctx, calc, projector_uid, proj_data),
            Self::BombApplication => get_bomb_application_mult(ctx, calc, projector_uid, projectee_uid),
            Self::MissileOrBombApplication => {
                get_missile_or_bomb_application_mult(ctx, calc, projector_uid, projectee_uid, proj_data)
            }
            Self::Neut => get_neut_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::AoeDdDmg => get_aoe_dd_dmg_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
            Self::AoeDdSideNeut => get_aoe_dd_side_neut_proj_mult(ctx, calc, projector_uid, projectee_uid, proj_data),
            Self::AoeBurst => get_aoe_burst_proj_mult(ctx, calc, projector_uid, effect, projectee_uid, proj_data),
            Self::AoeBurstRange => get_aoe_burst_range_mult(ctx, calc, projector_uid, effect, proj_data),
        }
    }
}
