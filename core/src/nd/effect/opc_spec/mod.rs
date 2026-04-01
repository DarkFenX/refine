pub(crate) use base_output::{
    NEffectBreacherAmount, NEffectBreacherOutputGetter, NEffectDmgOutputGetter, NEffectEcmAmount,
    NEffectEcmOutputGetter, NEffectGeneralOutputGetter, NEffectMiningAmount, NEffectMiningOutputGetter,
    NEffectMiningXargs, NEffectOutputGetter,
};
pub(crate) use charge_mult_getter::NEffectChargeMultGetter;
pub(crate) use opc_spec::{NEffectLocalOpcSpec, NEffectProjOpcSpec};
pub(crate) use proj_mult::NEffectProjMultGetter;
pub(crate) use resist::NEffectResist;

mod base_output;
mod charge_mult_getter;
mod opc_spec;
mod proj_mult;
mod resist;
