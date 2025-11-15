use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    misc::MiningAmount,
    nd::{NEffect, NEffectHc, eff::shared::mining_opc},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::MINING;
const A_EFFECT_ID: AEffectId = ac::effects::MINING;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            mining_ore_opc_getter: Some(get_mining_ore_opc),
            mining_ice_opc_getter: Some(get_mining_ice_opc),
            ..
        },
        ..
    }
}

fn get_mining_ore_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_key);
    if item.get_axt()?.is_ice_harvester {
        return None;
    }
    mining_opc::get_mining_opc(ctx, calc, item_key, effect)
}

fn get_mining_ice_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_key);
    if !item.get_axt()?.is_ice_harvester {
        return None;
    }
    mining_opc::get_mining_opc(ctx, calc, item_key, effect)
}
