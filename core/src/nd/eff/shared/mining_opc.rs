use crate::{
    ac,
    def::{AttrVal, OF},
    misc::MiningAmount,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

pub(in crate::nd::eff) fn get_mining_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let (delay, yield_, waste) = get_mining_values(ctx, calc, item_key, effect)?;
    Some(Output::Simple(OutputSimple {
        amount: MiningAmount { yield_, waste },
        delay,
    }))
}

pub(in crate::nd::eff) fn get_mining_values(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<(AttrVal, AttrVal, AttrVal)> {
    let delay = eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect)?;
    let yield_ = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::MINING_AMOUNT)?;
    let waste_chance = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::MINING_WASTE_PROBABILITY)?;
    let waste = match waste_chance > OF(0.0) {
        true => {
            let waste_mult = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::MINING_WASTED_VOLUME_MULT)?;
            waste_chance * yield_ * waste_mult / OF(100.0)
        }
        false => OF(0.0),
    };
    Some((delay, yield_, waste))
}
