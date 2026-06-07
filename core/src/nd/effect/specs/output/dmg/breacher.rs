use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue, UnitInterval, Value},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output amount
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) struct NEffectBreacherAmount {
    // Absolute damage cap per tick/instance of damage
    pub(crate) absolute_max: PValue,
    // Relative damage cap per tick/instance of damage
    pub(crate) relative_max: UnitInterval,
    pub(crate) tick_count: Count,
}
impl NEffectBreacherAmount {
    pub(crate) fn try_new(absolute_max: PValue, relative_max: UnitInterval, tick_count: Count) -> Option<Self> {
        if tick_count == Count::ZERO {
            return None;
        }
        Some(Self {
            absolute_max,
            relative_max,
            tick_count,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectBreacherOutputGetter {
    Regular,
}
impl NEffectOutputGetter for NEffectBreacherOutputGetter {
    type Instance = NEffectBreacherAmount;
    type XArgs = ();

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        _effect: &REffect,
        _xargs: Self::XArgs,
    ) -> Option<Output<Self::Instance>> {
        match self {
            Self::Regular => get_regular(ctx, calc, item_uid),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter-related private functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_regular(ctx: SvcCtx, calc: &mut Calc, projector_uid: UItemId) -> Option<Output<NEffectBreacherAmount>> {
    let abs_max = PValue::from_value_clamped(calc.get_item_oattr_afb_oextra(
        ctx,
        projector_uid,
        ctx.ac().dot_max_dmg_per_tick,
        Value::ZERO,
    )?);
    let rel_max = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, projector_uid, ctx.ac().dot_max_hp_perc_per_tick, Value::ZERO)?
            / Value::HUNDRED,
    );
    let duration_s = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, projector_uid, ctx.ac().dot_duration, Value::ZERO)? / Value::THOUSAND,
    );
    let breacher = NEffectBreacherAmount::try_new(
        abs_max,
        UnitInterval::from_pvalue_clamped(rel_max),
        Count::from_pvalue_trunced(duration_s * PValue::SERVER_TICK_HZ),
    )?;
    Some(Output::Simple(OutputSimple {
        instance: breacher,
        delay: PValue::ZERO,
    }))
}
