use crate::{
    def::{AttrVal, OF},
    misc::MiningAmount,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
    util::FLOAT_TOLERANCE,
};

pub(in crate::nd::effect::data) fn get_mining_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_key, effect)?;
    Some(Output::Simple(OutputSimple {
        amount: MiningAmount { yield_, drain },
        delay,
    }))
}

pub(in crate::nd::effect::data) fn get_crit_mining_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
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

fn get_mining_values(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    effect: &REffect,
) -> Option<(AttrVal, AttrVal, AttrVal)> {
    let delay = funcs::get_effect_duration_s(ctx, calc, item_key, effect)?;
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

////////////////////////////////////////////////////////////////////////////////////////////////////
