use crate::{
    def::AttrVal,
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

pub(crate) type NEffectLocalBaseOutputGetter<T> = fn(SvcCtx, &mut Calc, UItemKey, &REffect) -> Option<Output<T>>;
pub(crate) type NEffectChargeMultGetter = fn(SvcCtx, &mut Calc, UItemKey, AttrVal) -> Option<AttrVal>;
pub(crate) type NEffectInstanceLimitGetter = fn(SvcCtx, &mut Calc, UItemKey) -> Option<AttrVal>;

#[derive(Copy, Clone)]
pub(crate) struct NEffectLocalOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NEffectLocalBaseOutputGetter<T>,
    pub(crate) charge_mult: Option<NEffectChargeMultGetter> = None,
    pub(crate) instance_limit: Option<NEffectInstanceLimitGetter> = None,
}
