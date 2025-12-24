use crate::{
    def::AttrVal,
    nd::{NProjMultGetter, NSpoolGetter},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

pub(crate) type NBaseOutputGetter<T> = fn(SvcCtx, &mut Calc, UItemKey, &REffect) -> Option<Output<T>>;
pub(crate) type NChargeMultGetter = fn(SvcCtx, &mut Calc, UItemKey, AttrVal) -> Option<AttrVal>;
pub(crate) type NLocalInstanceLimitGetter = fn(SvcCtx, &mut Calc, UItemKey) -> Option<AttrVal>;
pub(crate) type NProjInstanceLimitGetter = fn(SvcCtx, &mut Calc, UItemKey) -> Option<AttrVal>;

#[derive(Copy, Clone)]
pub(crate) struct NEffectLocalOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NBaseOutputGetter<T>,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) instance_limit: Option<NLocalInstanceLimitGetter> = None,
}

#[derive(Copy, Clone)]
pub(crate) struct NEffectProjOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NBaseOutputGetter<T>,
    pub(crate) proj_mult: NProjMultGetter,
    pub(crate) spool: Option<NSpoolGetter> = None,
    pub(crate) charge_mult: Option<NChargeMultGetter> = None,
    pub(crate) instance_limit: Option<NProjInstanceLimitGetter> = None,
}
