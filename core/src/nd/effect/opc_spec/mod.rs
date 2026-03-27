pub(crate) use base_output::{
    NBreacherOutputGetter, NDmgOutputGetter, NEcmOutputGetter, NGeneralOutputGetter, NMiningOutputGetter, NMiningXargs,
    NOutputGetter,
};
pub(crate) use charge_mult_getter::NChargeMultGetter;
pub(crate) use opc_spec::{NEffectLocalOpcSpec, NEffectProjOpcSpec};
pub(crate) use proj_mult::NEffectProjMultGetter;
pub(crate) use resist::NEffectResist;

mod base_output;
mod charge_mult_getter;
mod opc_spec;
mod proj_mult;
mod resist;
