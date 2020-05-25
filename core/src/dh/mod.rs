//! Definition of data handling interface

pub use data::{
    Container, DgmAttr, DgmBuff, DgmBuffIM, DgmBuffLGM, DgmBuffLM, DgmBuffLRSM, DgmEffect, DgmEffectMod, DgmMutaAttr,
    DgmMutaType, DgmTypeAttr, DgmTypeEffect, FtrAbil, FtrTypeAbil, FtrTypeAbilChargeData, FtrTypeAbilData, InvGroup,
    InvType, Primitive, SkillReq,
};
pub use handler::{DataHandler, Result};

mod data;
mod handler;
