use crate::{
    def::AttrVal,
    nd::{NProjMultGetter, NSpoolRaw},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemKey,
};

pub(crate) type NEffectBaseOutputGetter<T> = fn(SvcCtx, &mut Calc, UItemKey, &REffect) -> Option<Output<T>>;
pub(crate) type NEffectSpoolGetter = fn(SvcCtx, &mut Calc, UItemKey) -> Option<NSpoolRaw>;
pub(crate) type NEffectChargeMultGetter = fn(SvcCtx, &mut Calc, UItemKey, AttrVal) -> Option<AttrVal>;
pub(crate) type NEffectLocalInstanceLimitGetter = fn(SvcCtx, &mut Calc, UItemKey) -> Option<AttrVal>;
pub(crate) type NEffectProjInstanceLimitGetter = fn(SvcCtx, &mut Calc, UItemKey) -> Option<AttrVal>;

#[derive(Copy, Clone)]
pub(crate) struct NEffectLocalOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NEffectBaseOutputGetter<T>,
    pub(crate) charge_mult: Option<NEffectChargeMultGetter> = None,
    pub(crate) instance_limit: Option<NEffectLocalInstanceLimitGetter> = None,
}

#[derive(Copy, Clone)]
pub(crate) struct NEffectProjOpcSpec<T>
where
    T: Copy
{
    pub(crate) base: NEffectBaseOutputGetter<T>,
    pub(crate) proj_mult: NProjMultGetter,
    pub(crate) spool: Option<NEffectSpoolGetter> = None,
    pub(crate) charge_mult: Option<NEffectChargeMultGetter> = None,
    pub(crate) instance_limit: Option<NEffectProjInstanceLimitGetter> = None,
}
