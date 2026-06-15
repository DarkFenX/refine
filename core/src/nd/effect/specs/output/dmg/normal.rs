use crate::{
    misc::DmgKinds,
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{RAttrId, REffect},
    svc::{
        SvcCtx,
        calc::Calc,
        funcs,
        output::{Output, OutputComplex, OutputSimple},
    },
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectDmgOutputGetter {
    Regular,
    MultCharge,
    Delay1,
    Delay2,
    DotDelay,
    // Variants specific to a single effect
    TargetAttack,
    Bomb,
    FtrAbilAttackM,
    FtrAbilMissiles,
    FtrAbilKamikaze,
}
impl NEffectOutputGetter for NEffectDmgOutputGetter {
    type Instance = DmgKinds<PValue>;
    type XArgs = ();

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        _xargs: Self::XArgs,
    ) -> Option<Output<Self::Instance>> {
        match self {
            Self::Regular => get_regular(ctx, calc, item_uid),
            Self::MultCharge => get_mult_charge(ctx, calc, item_uid),
            Self::Delay1 => get_delay1(ctx, calc, item_uid),
            Self::Delay2 => get_delay2(ctx, calc, item_uid),
            Self::DotDelay => get_dot_delay(ctx, calc, item_uid),
            // Variants specific to a single effect
            Self::TargetAttack => get_target_attack(ctx, calc, item_uid),
            Self::Bomb => get_bomb(ctx, calc, item_uid),
            Self::FtrAbilAttackM => get_ftr_abil_attack_m(ctx, calc, item_uid),
            Self::FtrAbilMissiles => get_ftr_abil_missiles(ctx, calc, item_uid),
            Self::FtrAbilKamikaze => get_ftr_abil_kamikaze(ctx, calc, item_uid, effect),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter-related private functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_regular(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    Some(Output::Simple(OutputSimple {
        instance: get_dmg_values_standard(ctx, calc, item_uid)?,
        delay: PValue::ZERO,
    }))
}

fn get_delay1(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    Some(Output::Simple(OutputSimple {
        instance: get_dmg_values_standard(ctx, calc, item_uid)?,
        delay: PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().dmg_delay_duration, Value::ZERO)? / Value::THOUSAND,
        ),
    }))
}

fn get_delay2(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    Some(Output::Simple(OutputSimple {
        instance: get_dmg_values_standard(ctx, calc, item_uid)?,
        delay: PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
                / Value::THOUSAND,
        ),
    }))
}

fn get_dot_delay(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    let dmg = get_dmg_values_standard(ctx, calc, item_uid)?;
    let delay = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
            / Value::THOUSAND,
    );
    if let Some(interval_ms) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().doomsday_dmg_cycle_time)
        && interval_ms > Value::FLOAT_TOLERANCE
        && let Some(duration_ms) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().doomsday_dmg_duration)
    {
        let repeats = Count::from_value_trunced(duration_ms / interval_ms);
        if repeats > Count::ONE {
            return Some(Output::Complex(OutputComplex {
                instance: dmg,
                delay,
                repeats,
                interval: PValue::from_value_clamped(interval_ms / Value::THOUSAND),
            }));
        }
    }
    Some(Output::Simple(OutputSimple { instance: dmg, delay }))
}

fn get_mult_charge(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    let charge_uid = ctx.u_data.items.get(item_uid).get_charge_uid()?;
    let dmg_mult =
        PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().dmg_mult, Value::ONE)?);
    let mut dmg = get_dmg_values_standard(ctx, calc, charge_uid)?;
    dmg.em *= dmg_mult;
    dmg.thermal *= dmg_mult;
    dmg.kinetic *= dmg_mult;
    dmg.explosive *= dmg_mult;
    Some(Output::Simple(OutputSimple {
        instance: dmg,
        delay: PValue::ZERO,
    }))
}

fn get_target_attack(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    let item = ctx.u_data.items.get(item_uid);
    let dmg_dealer_uid = match item.get_axt().unwrap().capacity > PValue::ZERO {
        // If item has capacity but no charge - it is not dealing damage
        true => item.get_charge_uid()?,
        false => item_uid,
    };
    let dmg_mult =
        PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().dmg_mult, Value::ONE)?);
    let mut dmg = get_dmg_values_standard(ctx, calc, dmg_dealer_uid)?;
    dmg.em *= dmg_mult;
    dmg.thermal *= dmg_mult;
    dmg.kinetic *= dmg_mult;
    dmg.explosive *= dmg_mult;
    Some(Output::Simple(OutputSimple {
        instance: dmg,
        delay: PValue::ZERO,
    }))
}

// The only difference from regular getter is that bomb damage can be modified by fighter count
fn get_bomb(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    let mut dmg = get_dmg_values_standard(ctx, calc, item_uid)?;
    if let Some(mult) = ctx.u_data.get_charge_mult(item_uid) {
        dmg.em *= mult;
        dmg.thermal *= mult;
        dmg.kinetic *= mult;
        dmg.explosive *= mult;
    }
    Some(Output::Simple(OutputSimple {
        instance: dmg,
        delay: PValue::ZERO,
    }))
}

fn get_ftr_abil_attack_m(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    let mut dmg = get_dmg_values(
        ctx,
        calc,
        item_uid,
        ctx.ac().ftr_abil_atk_missile_dmg_em,
        ctx.ac().ftr_abil_atk_missile_dmg_therm,
        ctx.ac().ftr_abil_atk_missile_dmg_kin,
        ctx.ac().ftr_abil_atk_missile_dmg_expl,
    )?;
    let mut dmg_mult = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        item_uid,
        ctx.ac().ftr_abil_atk_missile_dmg_mult,
        Value::ONE,
    )?);
    if let Ok(u_fighter) = ctx.u_data.items.get(item_uid).dc_fighter()
        && let Some(count) = u_fighter.get_count()
    {
        dmg_mult *= count.into_pvalue();
    }
    dmg.em *= dmg_mult;
    dmg.thermal *= dmg_mult;
    dmg.kinetic *= dmg_mult;
    dmg.explosive *= dmg_mult;
    Some(Output::Simple(OutputSimple {
        instance: dmg,
        delay: PValue::ZERO,
    }))
}

fn get_ftr_abil_missiles(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    let mut dmg = get_dmg_values(
        ctx,
        calc,
        item_uid,
        ctx.ac().ftr_abil_missiles_dmg_em,
        ctx.ac().ftr_abil_missiles_dmg_therm,
        ctx.ac().ftr_abil_missiles_dmg_kin,
        ctx.ac().ftr_abil_missiles_dmg_expl,
    )?;
    let mut dmg_mult = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        item_uid,
        ctx.ac().ftr_abil_missiles_dmg_mult,
        Value::ONE,
    )?);
    if let Ok(u_fighter) = ctx.u_data.items.get(item_uid).dc_fighter()
        && let Some(count) = u_fighter.get_count()
    {
        dmg_mult *= count.into_pvalue();
    }
    dmg.em *= dmg_mult;
    dmg.thermal *= dmg_mult;
    dmg.kinetic *= dmg_mult;
    dmg.explosive *= dmg_mult;
    Some(Output::Simple(OutputSimple {
        instance: dmg,
        delay: PValue::ZERO,
    }))
}

fn get_ftr_abil_kamikaze(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<Output<DmgKinds<PValue>>> {
    let mut dmg = get_dmg_values(
        ctx,
        calc,
        item_uid,
        ctx.ac().ftr_abil_kamikaze_dmg_em,
        ctx.ac().ftr_abil_kamikaze_dmg_therm,
        ctx.ac().ftr_abil_kamikaze_dmg_kin,
        ctx.ac().ftr_abil_kamikaze_dmg_expl,
    )?;
    if let Ok(u_fighter) = ctx.u_data.items.get(item_uid).dc_fighter()
        && let Some(count) = u_fighter.get_count()
    {
        let dmg_mult = count.into_pvalue();
        dmg.em *= dmg_mult;
        dmg.thermal *= dmg_mult;
        dmg.kinetic *= dmg_mult;
        dmg.explosive *= dmg_mult;
    }
    Some(Output::Simple(OutputSimple {
        instance: dmg,
        delay: PValue::ZERO,
    }))
}

fn get_dmg_values_standard(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<DmgKinds<PValue>> {
    get_dmg_values(
        ctx,
        calc,
        item_uid,
        ctx.ac().em_dmg,
        ctx.ac().therm_dmg,
        ctx.ac().kin_dmg,
        ctx.ac().expl_dmg,
    )
}
fn get_dmg_values(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    em_attr_rid: Option<RAttrId>,
    therm_attr_rid: Option<RAttrId>,
    kin_attr_rid: Option<RAttrId>,
    expl_attr_rid: Option<RAttrId>,
) -> Option<DmgKinds<PValue>> {
    Some(DmgKinds {
        em: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(ctx, item_uid, em_attr_rid, Value::ZERO)?),
        thermal: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            therm_attr_rid,
            Value::ZERO,
        )?),
        kinetic: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            kin_attr_rid,
            Value::ZERO,
        )?),
        explosive: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            expl_attr_rid,
            Value::ZERO,
        )?),
    })
}
