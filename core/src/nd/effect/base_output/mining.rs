use crate::{
    misc::MiningAmount,
    nd::NBaseOutputGetter,
    num::{PValue, UnitInterval, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
};

#[derive(Copy, Clone)]
pub(crate) struct NMiningXargs {
    pub(crate) mission_ore: bool,
}

pub(crate) enum NBaseMiningGetter {
    Regular,
    RegularHybridOre,
    RegularHybridIce,
    CritHybridOre,
    CritHybridIce,
}
impl NBaseOutputGetter for NBaseMiningGetter {
    type Instance = MiningAmount;
    type Xargs = NMiningXargs;

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
            Self::RegularHybridOre => get_regular_hybrid_ore(ctx, calc, item_uid, effect, xargs),
            Self::RegularHybridIce => get_regular_hybrid_ice(ctx, calc, item_uid, effect, xargs),
            Self::CritHybridOre => get_crit_hybrid_ore(ctx, calc, item_uid, effect, xargs),
            Self::CritHybridIce => get_crit_hybrid_ice(ctx, calc, item_uid, effect, xargs),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_regular_hybrid_ore(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    xargs: NMiningXargs,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_uid);
    if item.is_ice_harvester() {
        return None;
    }
    get_regular(ctx, calc, item_uid, effect, xargs)
}

fn get_regular_hybrid_ice(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    xargs: NMiningXargs,
) -> Option<Output<MiningAmount>> {
    let item = ctx.u_data.items.get(item_uid);
    if !item.is_ice_harvester() {
        return None;
    }
    get_regular(ctx, calc, item_uid, effect, xargs)
}

fn get_regular(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    xargs: NMiningXargs,
) -> Option<Output<MiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_uid, effect, xargs)?;
    Some(Output::Simple(OutputSimple {
        instance: MiningAmount { yield_, drain },
        delay,
    }))
}

fn get_crit_hybrid_ore(
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
    get_crit(ctx, calc, item_uid, effect, base_xargs)
}

fn get_crit_hybrid_ice(
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
    get_crit(ctx, calc, item_uid, effect, base_xargs)
}

fn get_crit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    xargs: NMiningXargs,
) -> Option<Output<MiningAmount>> {
    let (delay, yield_, drain) = get_mining_values(ctx, calc, item_uid, effect, xargs)?;
    let attr_consts = ctx.ac();
    // Mission ore is immune to crits
    let crit_chance = match xargs.mission_ore {
        true => Value::ZERO,
        false => calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.mining_crit_chance, Value::ZERO)?,
    };
    let yield_ = match crit_chance > Value::FLOAT_TOLERANCE {
        true => {
            let crit_bonus =
                calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.mining_crit_bonus_yield, Value::ZERO)?;
            let crit_chance = UnitInterval::from_value_clamped(crit_chance);
            PValue::from_value_clamped(yield_ * (crit_chance.into_value().mul_add(crit_bonus, Value::ONE)))
        }
        false => yield_,
    };
    Some(Output::Simple(OutputSimple {
        instance: MiningAmount { yield_, drain },
        delay,
    }))
}

fn get_mining_values(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    xargs: NMiningXargs,
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
