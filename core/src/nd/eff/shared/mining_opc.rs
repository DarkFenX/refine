use crate::{
    ac,
    def::OF,
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
    crits: bool,
) -> Option<Output<MiningAmount>> {
    let delay = eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect)?;
    let yield_amount = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::MINING_AMOUNT)?;
    let waste_chance =
        calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::MINING_WASTE_PROBABILITY)? / OF(100.0);
    let waste_mult = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::MINING_WASTED_VOLUME_MULT)?;
    Some(Output::Simple(OutputSimple {
        amount: MiningAmount {
            yield_: yield_amount,
            waste: waste_chance * yield_amount * waste_mult,
        },
        delay,
    }))
}
