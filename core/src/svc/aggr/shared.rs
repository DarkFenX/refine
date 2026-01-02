use crate::{
    def::{AttrVal, OF},
    nd::NChargeMultGetter,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(super) fn calc_charge_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    charge_mult_getter: Option<NChargeMultGetter>,
    cycle_chargedness: Option<AttrVal>,
) -> Option<AttrVal> {
    match charge_mult_getter {
        Some(charge_mult_getter) if let Some(chargedness) = cycle_chargedness => {
            charge_mult_getter(ctx, calc, item_key, chargedness).and_then(|v| process_mult(v))
        }
        _ => None,
    }
}

pub(super) fn process_mult(mult: AttrVal) -> Option<AttrVal> {
    match mult {
        OF(1.0) => None,
        v => Some(v),
    }
}

pub(in crate::svc) struct AggrAmount<T> {
    pub(in crate::svc) amount: T,
    pub(in crate::svc) time: AttrVal,
}
impl<T> AggrAmount<T>
where
    T: std::ops::Div<AttrVal, Output = T>,
{
    pub(super) fn get_ps(self) -> Option<T> {
        if self.time == OF(0.0) {
            return None;
        }
        Some(self.amount / self.time)
    }
}

pub(in crate::svc) struct AggrOutput<T>
where
    T: Copy,
{
    pub(in crate::svc) output: Output<T>,
    pub(in crate::svc) time: AttrVal,
}
