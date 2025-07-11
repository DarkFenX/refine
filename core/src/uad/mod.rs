//! User & adapted data.

pub(crate) use fit::{Fits, ItemVec, UadFit, UadFitSkill};
pub(crate) use fleet::UadFleet;
pub(crate) use item::{
    ShipKind, UadAutocharge, UadBooster, UadCharacter, UadCharge, UadDrone, UadFighter, UadFwEffect, UadImplant,
    UadItem, UadModule, UadProjEffect, UadProjRange, UadRig, UadService, UadShip, UadSkill, UadStance, UadSubsystem,
    UadSwEffect, get_combined_a_attr_values,
};
pub(crate) use uad::Uad;

mod container;
pub(crate) mod err;
mod fit;
mod fleet;
mod item;
mod uad;
mod uade_access;
mod uade_debug;
