use crate::{
    misc::{MiningAmount, PValue, UnitInterval, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
};

pub(in crate::nd::effect::data) fn get_mining_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_uid, effect)?;
    Some(Output::Simple(OutputSimple {
        amount: MiningAmount { yield_, drain },
        delay,
    }))
}

pub(in crate::nd::effect::data) fn get_crit_mining_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<Output<MiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_uid, effect)?;
    let attr_consts = ctx.ac();
    let crit_chance = calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.mining_crit_chance, Value::ZERO)?;
    let yield_ = match crit_chance > Value::FLOAT_TOLERANCE {
        true => {
            let crit_bonus =
                calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.mining_crit_bonus_yield, Value::ZERO)?;
            let crit_chance = UnitInterval::from_value_clamped(crit_chance);
            PValue::from_val_clamped(yield_ * (Value::ONE + crit_bonus * crit_chance))
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
    item_uid: UItemId,
    effect: &REffect,
) -> Option<(PValue, PValue, PValue)> {
    let delay = funcs::get_effect_duration_s(ctx, calc, item_uid, effect)?;
    let yield_ =
        PValue::from_val_clamped(calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mining_amount, Value::ZERO)?);
    let waste_chance_perc =
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mining_waste_probability, Value::ZERO)?;
    let waste = match waste_chance_perc > Value::FLOAT_TOLERANCE {
        true => {
            let waste_mult = PValue::from_val_clamped(calc.get_item_oattr_afb_oextra(
                ctx,
                item_uid,
                ctx.ac().mining_wasted_volume_mult,
                Value::ZERO,
            )?);
            let waste_chance = UnitInterval::from_value_clamped(waste_chance_perc / Value::HUNDRED);
            yield_ * waste_mult * waste_chance
        }
        false => PValue::ZERO,
    };
    Some((delay, yield_, yield_ + waste))
}
