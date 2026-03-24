use crate::{
    ad::AEffectId,
    ed::EEffectId,
    misc::MiningAmount,
    nd::{
        NEffect, NEffectProjMultGetter, NEffectProjOpcSpec, NMiningXargs,
        effect::data::shared::base_opc::get_mining_base_opc,
    },
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

const EFFECT_EID: EEffectId = EEffectId::MINING;
const EFFECT_AID: AEffectId = AEffectId::MINING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        mining_ore_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_ore_mining_base_opc,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        mining_ice_opc_spec: Some(NEffectProjOpcSpec {
            base: internal_get_ice_mining_base_opc,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}

fn internal_get_ore_mining_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    base_xargs: NMiningXargs,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_uid);
    if item.is_ice_harvester() {
        return None;
    }
    get_mining_base_opc(ctx, calc, item_uid, effect, base_xargs)
}

fn internal_get_ice_mining_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    base_xargs: NMiningXargs,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_uid);
    if !item.is_ice_harvester() {
        return None;
    }
    get_mining_base_opc(ctx, calc, item_uid, effect, base_xargs)
}
