use crate::{
    misc::Breacher,
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

#[derive(Copy, Clone)]
pub(crate) enum NEffectBreacherOutputGetter {
    Regular,
}
impl NEffectOutputGetter for NEffectBreacherOutputGetter {
    type Instance = Breacher;
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
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_regular(ctx: SvcCtx, calc: &mut Calc, projector_uid: UItemId) -> Option<Output<Breacher>> {
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
    let breacher = Breacher::try_new(
        abs_max,
        UnitInterval::from_pvalue_clamped(rel_max),
        Count::from_pvalue_trunced(duration_s * PValue::SERVER_TICK_HZ),
    )?;
    Some(Output::Simple(OutputSimple {
        instance: breacher,
        delay: PValue::ZERO,
    }))
}
