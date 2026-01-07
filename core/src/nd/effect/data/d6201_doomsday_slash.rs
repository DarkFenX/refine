use crate::{
    ad::{AEffectBuff, AEffectId},
    ed::EEffectId,
    misc::{DmgKinds, PValue, Value},
    nd::{
        NEffect, NEffectDmgKind, NEffectProjOpcSpec,
        effect::data::shared::{
            base_opc::get_aoe_dd_side_neut_opc_spec, mods::make_dd_self_debuffs, proj_mult::get_aoe_dd_dmg_proj_mult,
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

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_SLASH;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_SLASH;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_dmg_base_opc,
            proj_mult_str: Some(get_aoe_dd_dmg_proj_mult),
            ..
        }),
        neut_opc_spec: Some(get_aoe_dd_side_neut_opc_spec()),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}

fn internal_get_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<DmgKinds<PValue>>> {
    // Unlike other AoE doomsdays, reapers hit every ship only once, despite having damage ticks
    // spread over time. We also assume target is hit by first damage tick.
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
                ctx,
                item_uid,
                ctx.ac().em_dmg,
                Value::ZERO,
            )?),
            thermal: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
                ctx,
                item_uid,
                ctx.ac().therm_dmg,
                Value::ZERO,
            )?),
            kinetic: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
                ctx,
                item_uid,
                ctx.ac().kin_dmg,
                Value::ZERO,
            )?),
            explosive: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
                ctx,
                item_uid,
                ctx.ac().expl_dmg,
                Value::ZERO,
            )?),
        },
        delay: PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
                / Value::THOUSAND,
        ),
    }))
}
