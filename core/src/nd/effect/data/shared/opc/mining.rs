use crate::{
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
    util::FLOAT_TOLERANCE,
};

pub(in crate::nd::effect::data) fn get_mining_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_key, effect)?;
    Some(Output::Simple(OutputSimple {
        amount: MiningAmount { yield_, drain },
        delay,
    }))
}

pub(in crate::nd::effect::data) fn get_mining_values(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<(AttrVal, AttrVal, AttrVal)> {
    let delay = eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect)?;
    let attr_consts = ctx.ac();
    let yield_ = calc.get_item_oattr_afb_oextra(ctx, item_key, attr_consts.mining_amount, OF(0.0))?;
    let waste_chance = calc.get_item_oattr_afb_oextra(ctx, item_key, attr_consts.mining_waste_probability, OF(0.0))?;
    let waste = match waste_chance > FLOAT_TOLERANCE {
        true => {
            let waste_mult =
                calc.get_item_oattr_afb_oextra(ctx, item_key, attr_consts.mining_wasted_volume_mult, OF(0.0))?;
            yield_ * waste_mult * (waste_chance / OF(100.0)).clamp(OF(0.0), OF(1.0))
        }
        false => OF(0.0),
    };
    Some((delay, yield_, yield_ + waste))
}
