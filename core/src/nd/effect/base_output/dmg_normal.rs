use crate::{
    misc::DmgKinds,
    nd::NBaseOutputGetter,
    num::{Count, PValue, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputComplex, OutputSimple},
    },
    ud::UItemId,
};

pub(crate) enum NBaseNormalDmgGetter {
    Regular,
    MultCharge,
    Delay1,
    Delay2,
    DotDelay,
    // Variants specific to a single effect
    TargetAttack,
}
impl NBaseOutputGetter for NBaseNormalDmgGetter {
    type Instance = DmgKinds<PValue>;
    type Xargs = ();

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        _effect: &REffect,
        _xargs: Self::Xargs,
    ) -> Option<Output<Self::Instance>> {
        match self {
            Self::Regular => get_regular(ctx, calc, item_uid),
            Self::MultCharge => get_mult_charge(ctx, calc, item_uid),
            Self::Delay1 => get_delay1(ctx, calc, item_uid),
            Self::Delay2 => get_delay2(ctx, calc, item_uid),
            Self::DotDelay => get_dot_delay(ctx, calc, item_uid),
            // Variants specific to a single effect
            Self::TargetAttack => get_target_attack(ctx, calc, item_uid),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_regular(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    Some(Output::Simple(OutputSimple {
        instance: get_dmg_values(ctx, calc, item_uid)?,
        delay: PValue::ZERO,
    }))
}

fn get_delay1(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    Some(Output::Simple(OutputSimple {
        instance: get_dmg_values(ctx, calc, item_uid)?,
        delay: PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().dmg_delay_duration, Value::ZERO)? / Value::THOUSAND,
        ),
    }))
}

fn get_delay2(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    Some(Output::Simple(OutputSimple {
        instance: get_dmg_values(ctx, calc, item_uid)?,
        delay: PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
                / Value::THOUSAND,
        ),
    }))
}

fn get_dot_delay(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    let dmg = get_dmg_values(ctx, calc, item_uid)?;
    let delay_s = PValue::from_value_clamped(
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
                delay: delay_s,
                repeats,
                interval: PValue::from_value_clamped(interval_ms / Value::THOUSAND),
            }));
        }
    }
    Some(Output::Simple(OutputSimple {
        instance: dmg,
        delay: delay_s,
    }))
}

fn get_mult_charge(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<Output<DmgKinds<PValue>>> {
    let charge_uid = ctx.u_data.items.get(item_uid).get_charge_uid()?;
    let dmg_mult =
        PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().dmg_mult, Value::ONE)?);
    let mut dmg = get_dmg_values(ctx, calc, charge_uid)?;
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
    let mut dmg = get_dmg_values(ctx, calc, dmg_dealer_uid)?;
    dmg.em *= dmg_mult;
    dmg.thermal *= dmg_mult;
    dmg.kinetic *= dmg_mult;
    dmg.explosive *= dmg_mult;
    Some(Output::Simple(OutputSimple {
        instance: dmg,
        delay: PValue::ZERO,
    }))
}

fn get_dmg_values(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<DmgKinds<PValue>> {
    Some(DmgKinds {
        em: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().em_dmg, Value::ZERO)?),
        thermal: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().therm_dmg,
            Value::ZERO,
        )?),
        kinetic: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().kin_dmg,
            Value::ZERO,
        )?),
        explosive: PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().expl_dmg,
            Value::ZERO,
        )?),
    })
}
