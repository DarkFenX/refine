//! User & adapted data.

pub(crate) use fit::{UadFit, UadFitSkill, UadFits, UadItemVec};
pub(crate) use fleet::UadFleet;
pub(crate) use item::{
    ShipKind, UadAutocharge, UadBooster, UadCharacter, UadCharge, UadDrone, UadEffectUpdates, UadFighter, UadFwEffect,
    UadImplant, UadItem, UadItems, UadModule, UadProjEffect, UadProjRange, UadRig, UadService, UadShip, UadSkill,
    UadStance, UadSubsystem, UadSwEffect, get_combined_a_attr_values,
};
pub(crate) use primitives::{UadFitKey, UadFleetKey, UadItemKey};
pub(crate) use uad::Uad;

mod container;
pub(crate) mod err;
mod fit;
mod fleet;
mod item;
mod primitives;
mod uad;
mod uade_access;
mod uade_debug;
