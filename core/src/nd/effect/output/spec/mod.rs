pub(crate) use base_getter::NEffectOutputGetter;
pub(crate) use charge_mult_getter::NEffectChargeMultGetter;
pub(crate) use proj_getter::NEffectProjGetter;
pub(crate) use resist::NEffectResist;
pub(crate) use spec_local::NEffectLocalOpcSpec;
pub(crate) use spec_proj::NEffectProjOpcSpec;

mod base_getter;
mod charge_mult_getter;
mod proj_getter;
mod resist;
mod spec_local;
mod spec_proj;
