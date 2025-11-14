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

const E_EFFECT_ID: EEffectId = ec::effects::MINING_CLOUDS;
const A_EFFECT_ID: AEffectId = ac::effects::MINING_CLOUDS;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            mining_gas_opc_getter: Some(internal_get_mining_opc),
            ..
        },
        ..
    }
}

fn internal_get_mining_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    mining_opc::get_mining_opc(ctx, calc, item_key, effect, false)
}
