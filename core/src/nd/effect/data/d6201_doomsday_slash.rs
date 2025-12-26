use crate::{
    ac,
    ad::{AEffectBuff, AEffectId},
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::DmgKinds,
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
    ud::{UItem, UItemKey},
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_SLASH;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_SLASH;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
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
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<DmgKinds<AttrVal>>> {
    // Unlike other AoE doomsdays, reapers hit every ship only once, despite having damage ticks
    // spread over time. We also assume target is hit by first damage tick.
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().em_dmg, OF(0.0))?,
            thermal: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().therm_dmg, OF(0.0))?,
            kinetic: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().kin_dmg, OF(0.0))?,
            explosive: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().expl_dmg, OF(0.0))?,
        },
        delay: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().doomsday_warning_duration, OF(0.0))? / OF(1000.0),
    }))
}
