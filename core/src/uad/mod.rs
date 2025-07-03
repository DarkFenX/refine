//! User & adapted data.

pub(crate) use fit::{FitSkill, Fits, ItemVec, UadFit};
pub(crate) use fleet::UadFleet;
pub(crate) use item::{
    ProjRange, ShipKind, UadAutocharge, UadBooster, UadCharacter, UadCharge, UadDrone, UadFighter, UadFwEffect,
    UadImplant, UadItem, UadModule, UadProjEffect, UadRig, UadService, UadShip, UadSkill, UadStance, UadSubsystem,
    UadSwEffect, get_combined_a_attr_values,
};
pub(crate) use uad::Uad;

mod container;
pub(crate) mod err;
mod fit;
mod fleet;
mod item;
mod uad;
mod uade_debug;
