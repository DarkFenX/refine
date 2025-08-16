pub(crate) use dmg_kind::NEffectDmgKind;
pub(crate) use effect_charge::{NEffectCharge, NEffectChargeDepl, NEffectChargeLoc};

pub(in crate::nd::eff) mod damp;
mod dmg_kind;
pub(in crate::nd::eff) mod dmg_opc;
mod effect_charge;
pub(in crate::nd::eff) mod missile_dmg_self_srq;
pub(in crate::nd::eff) mod proj_mult;
pub(in crate::nd::eff) mod prop_mods;
pub(in crate::nd::eff) mod rep_opc;
pub(in crate::nd::eff) mod sov_stability_generators;
pub(in crate::nd::eff) mod spool;
pub(in crate::nd::eff) mod subsystem_mods;
pub(in crate::nd::eff) mod util;
pub(in crate::nd::eff) mod wd;
pub(in crate::nd::eff) mod web;
