//! User data.
//!
//! Actually contains lots of non-user data for fast access to it, but primary role is still to
//! store user data, so it is called like that.

pub(crate) use data::UData;
pub use fit::FitId;
pub(crate) use fit::{UFit, UFitId, UFitSkill, UFits, UItemVec};
pub use fleet::FleetId;
pub(crate) use fleet::{UFleet, UFleetId, UFleets};
pub use item::ItemId;
pub(crate) use item::{
    UAttrMutationRequest, UAutocharge, UBooster, UCharacter, UCharge, UDrone, UEffectUpdates, UFighter, UFwEffect,
    UImplant, UItem, UItemId, UItemMutationRequest, UItems, UModule, UPhysics, UProjData, UProjEffect, UProjs, URig,
    UService, UShip, UShipKind, USkill, UStance, USubsystem, USwEffect, get_combined_attr_values,
};

mod container;
mod data;
mod datae_access;
mod datae_debug;
pub(crate) mod err;
mod fit;
mod fleet;
mod item;
