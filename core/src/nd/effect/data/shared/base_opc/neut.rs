use ordered_float::Float;

use super::generic::get_generic_base_opc;
use crate::{
    ac,
    def::{AttrVal, OF},
    nd::{NEffectProjOpcSpec, NEffectResist, effect::data::shared::proj_mult::get_aoe_dd_side_neut_proj_mult},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
    util::FLOAT_TOLERANCE,
};

pub(in crate::nd::effect::data) fn get_neut_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_generic_base_opc(ctx, calc, item_key, effect, ctx.ac().energy_neut_amount, true)
}

pub(in crate::nd::effect::data) fn get_nosf_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<AttrVal>> {
    // Not a blood raider ship - not considered as a neut
    if calc.get_item_oattr_oextra(ctx, item_key, ctx.ac().nos_override)?.abs() < FLOAT_TOLERANCE {
        return None;
    }
    get_generic_base_opc(ctx, calc, item_key, effect, ctx.ac().power_transfer_amount, false)
}

pub(in crate::nd::effect::data) fn get_aoe_neut_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<AttrVal>> {
    let attr_consts = ctx.ac();
    let amount = calc.get_item_oattr_afb_odogma(ctx, item_key, attr_consts.energy_neut_amount, OF(0.0))?;
    let delay = calc.get_item_oattr_afb_oextra(ctx, item_key, attr_consts.doomsday_warning_duration, OF(0.0))?;
    Some(Output::Simple(OutputSimple { amount, delay }))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// AoE doomsday side-effect neuting
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_aoe_dd_side_neut_opc_spec() -> NEffectProjOpcSpec<AttrVal> {
    NEffectProjOpcSpec {
        base: get_aoe_dd_side_neut_base_opc,
        proj_mult: get_aoe_dd_side_neut_proj_mult,
        resist: Some(NEffectResist::Attr(ac::attrs::DOOMSDAY_ENERGY_NEUT_RESIST_ID)),
        ilimit_attr_id: Some(ac::attrs::CAPACITOR_CAPACITY),
        ..
    }
}

fn get_aoe_dd_side_neut_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_generic_base_opc(ctx, calc, item_key, effect, ctx.ac().doomsday_energy_neut_amount, true)
}
