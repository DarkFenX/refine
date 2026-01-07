use crate::{
    ad::{AAttrId, AEffectId},
    ed::EEffectId,
    misc::{Ecm, PValue, Value},
    nd::{
        NEffect, NEffectDmgKind, NEffectProjOpcSpec, NEffectResist,
        effect::data::shared::{
            base_opc::get_instant_dmg_base_opc,
            proj_mult::{get_bomb_application_mult, get_bomb_range_mult},
        },
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemId},
};

const EFFECT_EID: EEffectId = EEffectId::BOMB_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::BOMB_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_dmg_base_opc,
            proj_mult_str: Some(get_bomb_application_mult),
            proj_mult_chance: Some(get_bomb_range_mult),
            ..
        }),
        neut_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_neut_base_opc,
            proj_mult_str: Some(get_bomb_application_mult),
            proj_mult_chance: Some(get_bomb_range_mult),
            resist: Some(NEffectResist::Standard),
            limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
            ..
        }),
        ecm_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_ecm_base_opc,
            proj_mult_chance: Some(get_bomb_range_mult),
            resist: Some(NEffectResist::Standard),
            ..
        }),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Bomb
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Neut
////////////////////////////////////////////////////////////////////////////////////////////////////
fn internal_get_neut_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<PValue>> {
    let amount = calc.get_item_oattr_afb_odogma(ctx, item_uid, ctx.ac().energy_neut_amount, Value::ZERO)?;
    let amount = match amount > Value::ZERO {
        true => PValue::from_value_unchecked(amount),
        // Do not return neut stats for non-neut bombs
        false => return None,
    };
    Some(Output::Simple(OutputSimple {
        amount,
        delay: PValue::ZERO,
    }))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ECM
////////////////////////////////////////////////////////////////////////////////////////////////////
fn internal_get_ecm_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    _projector_effect: &REffect,
) -> Option<Output<Ecm>> {
    let str_radar = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        projector_uid,
        ctx.ac().scan_radar_strength_bonus,
        Value::ZERO,
    )?);
    let str_magnet = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        projector_uid,
        ctx.ac().scan_magnetometric_strength_bonus,
        Value::ZERO,
    )?);
    let str_grav = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        projector_uid,
        ctx.ac().scan_gravimetric_strength_bonus,
        Value::ZERO,
    )?);
    let str_ladar = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        projector_uid,
        ctx.ac().scan_ladar_strength_bonus,
        Value::ZERO,
    )?);
    // Do not return ECM stats for non-ecm bombs
    if str_radar <= PValue::ZERO && str_magnet <= PValue::ZERO && str_grav <= PValue::ZERO && str_ladar <= PValue::ZERO
    {
        return None;
    }
    Some(Output::Simple(OutputSimple {
        amount: Ecm {
            radar: str_radar,
            magnetometric: str_magnet,
            gravimetric: str_grav,
            ladar: str_ladar,
            duration: PValue::ZERO,
        },
        delay: PValue::ZERO,
    }))
}
