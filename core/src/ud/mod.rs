//! User data.
//!
//! Actually contains lots of non-user data for fast access to it, but primary role is still to
//! store user data, so it is called like that.

pub(crate) use data::UData;
pub(crate) use fit::{UFit, UFitSkill, UFits, UItemVec};
pub(crate) use fleet::UFleet;
pub(crate) use item::{
    UAutocharge, UBooster, UCharacter, UCharge, UCoordinates, UDirection, UDrone, UEffectUpdates, UFighter, UFwEffect,
    UImplant, UItem, UItems, UModule, UPhysics, UProjData, UProjEffect, URig, UService, UShip, UShipKind, USkill,
    UStance, USubsystem, USwEffect, get_combined_attr_values,
};
pub(crate) use primitives::{UFitKey, UFleetKey, UItemKey};

mod container;
mod data;
mod datae_access;
mod datae_debug;
pub(crate) mod err;
mod fit;
mod fleet;
mod item;
mod primitives;
