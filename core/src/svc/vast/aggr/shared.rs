use crate::{
    nd::NChargeMultGetter,
    num::{PValue, UnitInterval},
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(super) fn calc_charge_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    charge_mult_getter: Option<NChargeMultGetter>,
    cycle_chargedness: Option<UnitInterval>,
) -> Option<PValue> {
    match charge_mult_getter {
        Some(charge_mult_getter) if let Some(chargedness) = cycle_chargedness => {
            charge_mult_getter(ctx, calc, item_uid, chargedness).and_then(|v| process_mult(v))
        }
        _ => None,
    }
}

pub(super) fn process_mult(mult: PValue) -> Option<PValue> {
    match mult {
        PValue::ONE => None,
        v => Some(v),
    }
}

pub(in crate::svc::vast) struct AggrAmount<T> {
    pub(in crate::svc::vast) amount: T,
    pub(in crate::svc::vast) duration: PValue,
}
impl<T> AggrAmount<T>
where
    T: std::ops::Div<PValue, Output = T>,
{
    pub(super) fn get_ps(self) -> Option<T> {
        if self.duration == PValue::ZERO {
            return None;
        }
        Some(self.amount / self.duration)
    }
}

pub(in crate::svc::vast) struct AggrOutput<T>
where
    T: Copy,
{
    pub(in crate::svc::vast) output: Output<T>,
    pub(in crate::svc::vast) duration: PValue,
}
