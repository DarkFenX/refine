//! Solar system item definitions.

use base::{SolItemBase, SolItemBaseMutable};
pub(in crate::sol) use container::SolItems;
pub(in crate::sol) use item::SolItem;
pub(in crate::sol) use item_autocharge::SolAutocharge;
pub(in crate::sol) use item_booster::SolBooster;
pub(in crate::sol) use item_character::SolCharacter;
pub(in crate::sol) use item_charge::SolCharge;
pub(in crate::sol) use item_drone::SolDrone;
pub(in crate::sol) use item_fighter::SolFighter;
pub(in crate::sol) use item_fw_effect::SolFwEffect;
pub(in crate::sol) use item_implant::SolImplant;
pub(in crate::sol) use item_module::SolModule;
pub use item_module::SolModuleState;
pub(in crate::sol) use item_proj_effect::SolProjEffect;
pub(in crate::sol) use item_rig::SolRig;
pub(in crate::sol) use item_ship::{SolShip, SolShipKind};
pub(in crate::sol) use item_skill::SolSkill;
pub(in crate::sol) use item_stance::SolStance;
pub(in crate::sol) use item_subsystem::SolSubsystem;
pub(in crate::sol) use item_sw_effect::SolSwEffect;
pub(in crate::sol) use misc::SolItemState;
use misc::{SolAutocharges, SolEffectModes, SolProjs, bool_to_state_active, bool_to_state_offline, state_to_bool};
pub use misc::{
    SolItemAddAttrMutation, SolItemAddMutation, SolItemAttrMutationValue, SolItemChangeAttrMutation, SolMinionState,
};

mod base;
mod container;
mod item;
mod item_autocharge;
mod item_booster;
mod item_character;
mod item_charge;
mod item_drone;
mod item_fighter;
mod item_fw_effect;
mod item_implant;
mod item_module;
mod item_proj_effect;
mod item_rig;
mod item_ship;
mod item_skill;
mod item_stance;
mod item_subsystem;
mod item_sw_effect;
mod misc;
