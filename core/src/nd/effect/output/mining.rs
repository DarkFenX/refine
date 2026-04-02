use crate::{
    nd::{NEffectOutputGetter, NEffectProjOpcSpec},
    num::{PValue, UnitInterval, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        funcs,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemId},
};

pub(crate) struct NEffectMining {
    pub(crate) checker: Option<NEffectMiningChecker>,
    pub(crate) ospec: NEffectProjOpcSpec<NEffectMiningOutputGetter>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Base item checker
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectMiningChecker {
    Ice,
    NonIce,
}
impl NEffectMiningChecker {
    pub(crate) fn check(&self, u_item: &UItem) -> bool {
        match self {
            Self::Ice => u_item.is_ice_harvester(),
            Self::NonIce => !u_item.is_ice_harvester(),
        }
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// Output amount and extra arguments
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) struct NEffectMiningAmount {
    pub(crate) yield_: PValue,
    pub(crate) drain: PValue,
}
impl NEffectMiningAmount {
    pub(crate) fn new() -> Self {
        Self {
            yield_: PValue::ZERO,
            drain: PValue::ZERO,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct NEffectMiningXargs {
    pub(crate) mission_ore: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectMiningOutputGetter {
    Regular,
    Crit,
}
impl NEffectOutputGetter for NEffectMiningOutputGetter {
    type Instance = NEffectMiningAmount;
    type Xargs = NEffectMiningXargs;

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        xargs: Self::Xargs,
    ) -> Option<Output<Self::Instance>> {
        match self {
            Self::Regular => get_regular(ctx, calc, item_uid, effect, xargs),
            Self::Crit => get_crit(ctx, calc, item_uid, effect, xargs),
        }
    }
}

fn get_regular(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    xargs: NEffectMiningXargs,
) -> Option<Output<NEffectMiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_uid, effect, xargs)?;
    Some(Output::Simple(OutputSimple {
        instance: NEffectMiningAmount { yield_, drain },
        delay,
    }))
}

fn get_crit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    xargs: NEffectMiningXargs,
) -> Option<Output<NEffectMiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_uid, effect, xargs)?;
    // Mission ore is immune to crits
    let crit_chance = match xargs.mission_ore {
        true => Value::ZERO,
        false => calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mining_crit_chance, Value::ZERO)?,
    };
    let yield_ = match crit_chance > Value::FLOAT_TOLERANCE {
        true => {
            let crit_bonus =
                calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mining_crit_bonus_yield, Value::ZERO)?;
            let crit_chance = UnitInterval::from_value_clamped(crit_chance);
            PValue::from_value_clamped(yield_ * (crit_chance.into_value().mul_add(crit_bonus, Value::ONE)))
        }
        false => yield_,
    };
    Some(Output::Simple(OutputSimple {
        instance: NEffectMiningAmount { yield_, drain },
        delay,
    }))
}

fn get_mining_values(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    xargs: NEffectMiningXargs,
) -> Option<(PValue, PValue, PValue)> {
    let delay = funcs::get_effect_duration_s(ctx, calc, item_uid, effect)?;
    let yield_ = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        item_uid,
        ctx.ac().mining_amount,
        Value::ZERO,
    )?);
    // Mission ore is immune to waste
    let waste_chance_perc = match xargs.mission_ore {
        true => Value::ZERO,
        false => calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().mining_waste_probability, Value::ZERO)?,
    };
    let waste = match waste_chance_perc > Value::FLOAT_TOLERANCE {
        true => {
            let waste_mult = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
                ctx,
                item_uid,
                ctx.ac().mining_wasted_volume_mult,
                Value::ZERO,
            )?);
            let waste_chance = UnitInterval::from_value_clamped(waste_chance_perc / Value::HUNDRED);
            yield_ * waste_mult * waste_chance.into_pvalue()
        }
        false => PValue::ZERO,
    };
    Some((delay, yield_, yield_ + waste))
}
