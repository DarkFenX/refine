use crate::{
    ac,
    ad::AEffectId,
    def::OF,
    ec,
    ed::EEffectId,
    misc::MiningAmount,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, effect::data::shared::opc::get_mining_values},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
    util::FLOAT_TOLERANCE,
};

const E_EFFECT_ID: EEffectId = ec::effects::MINING_LASER;
const A_EFFECT_ID: AEffectId = ac::effects::MINING_LASER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::Crystal {
                can_run_uncharged: true,
            }),
            activates_charge: false,
        }),
        mining_ore_opc_getter: Some(get_mining_ore_opc),
        mining_ice_opc_getter: Some(get_mining_ice_opc),
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
    if item.is_ice_harvester() {
        return None;
    }
    get_mining_opc(ctx, calc, item_key, effect)
}

fn get_mining_ice_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_key);
    if !item.is_ice_harvester() {
        return None;
    }
    get_mining_opc(ctx, calc, item_key, effect)
}

fn get_mining_opc(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, effect: &REffect) -> Option<Output<MiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_key, effect)?;
    let attr_consts = ctx.ac();
    let crit_chance = calc.get_item_oattr_afb_oextra(ctx, item_key, attr_consts.mining_crit_chance, OF(0.0))?;
    let yield_ = match crit_chance > FLOAT_TOLERANCE {
        true => {
            let crit_bonus =
                calc.get_item_oattr_afb_oextra(ctx, item_key, attr_consts.mining_crit_bonus_yield, OF(0.0))?;
            yield_ * (OF(1.0) + crit_chance.clamp(OF(0.0), OF(1.0)) * crit_bonus)
        }
        false => yield_,
    };
    Some(Output::Simple(OutputSimple {
        amount: MiningAmount { yield_, drain },
        delay,
    }))
}
