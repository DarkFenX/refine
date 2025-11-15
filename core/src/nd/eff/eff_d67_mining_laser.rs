use crate::{
    ac,
    ad::AEffectId,
    def::OF,
    ec,
    ed::EEffectId,
    misc::MiningAmount,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc, eff::shared::mining_opc},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::MINING_LASER;
const A_EFFECT_ID: AEffectId = ac::effects::MINING_LASER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::Crystal {
                    can_run_uncharged: true,
                }),
                activates_charge: false,
            }),
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
    get_mining_opc(ctx, calc, item_key, effect)
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
    get_mining_opc(ctx, calc, item_key, effect)
}

fn get_mining_opc(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey, effect: &REffect) -> Option<Output<MiningAmount>> {
    let (delay, yield_, waste) = mining_opc::get_mining_values(ctx, calc, item_key, effect)?;
    let crit_chance = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::MINING_CRIT_CHANCE)?;
    let yield_ = match crit_chance > OF(0.0) {
        true => {
            let crit_bonus = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::MINING_CRIT_BONUS_YIELD)?;
            yield_ * (OF(1.0) + crit_chance.clamp(OF(0.0), OF(1.0)) * crit_bonus)
        }
        false => yield_,
    };
    Some(Output::Simple(OutputSimple {
        amount: MiningAmount { yield_, waste },
        delay,
    }))
}
