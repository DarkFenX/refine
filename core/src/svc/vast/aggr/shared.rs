use crate::{
    nd::NChargeMultGetter,
    num::{PValue, UnitInterval},
    svc::{SvcCtx, calc::Calc, output::OutputInstanceIter},
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

pub(in crate::svc::vast) struct AggrIterItem<'a, T>
where
    T: Copy,
{
    pub(in crate::svc::vast) instance_iter: OutputInstanceIter<'a, T>,
    pub(in crate::svc::vast) cycle_duration: PValue,
}
