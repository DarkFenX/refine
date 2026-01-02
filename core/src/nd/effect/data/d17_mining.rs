use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    misc::MiningAmount,
    nd::{
        NEffect, NEffectProjOpcSpec,
        effect::data::shared::{base_opc::get_mining_base_opc, proj_mult::get_simple_s2s_noapp_proj_mult},
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

const E_EFFECT_ID: EEffectId = ec::effects::MINING;
const A_EFFECT_ID: AEffectId = ac::effects::MINING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        mining_ore_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_ore_mining_base_opc,
            proj_mult_str: Some(get_simple_s2s_noapp_proj_mult),
            ..
        }),
        mining_ice_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_ice_mining_base_opc,
            proj_mult_str: Some(get_simple_s2s_noapp_proj_mult),
            ..
        }),
        ..
    }
}

fn internal_get_ore_mining_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_key);
    if item.is_ice_harvester() {
        return None;
    }
    get_mining_base_opc(ctx, calc, item_key, effect)
}

fn internal_get_ice_mining_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_key);
    if !item.is_ice_harvester() {
        return None;
    }
    get_mining_base_opc(ctx, calc, item_key, effect)
}
