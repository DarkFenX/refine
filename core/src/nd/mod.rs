//! ND stands for eNtity Data.
//!
//! This module is a place for almost all the hardcoded customizations applied to various EVE
//! entities, and entities derived from them.

pub(crate) use eff::{
    N_EFFECT_MAP, N_EFFECTS, NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc, NLocalRepGetter, NProjMultGetter,
    NRemoteRepGetter,
};

mod eff;
