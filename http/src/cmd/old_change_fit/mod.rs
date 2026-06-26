pub(in crate::cmd) use fit::HChangeFitCmd;
pub(in crate::cmd) use item_autocharge::HChangeAutochargeCmd;
pub(in crate::cmd) use item_character::{
    HChangeCharacterCmd, HChangeCharacterViaFitIdCmd, HChangeCharacterViaItemIdCmd, HSetCharacterCmd,
    HUnsetCharacterCmd,
};
pub(in crate::cmd) use item_charge::HChangeChargeCmd;
pub(in crate::cmd) use item_fighter::{HAddFighterCmd, HChangeFighterCmd};
pub(in crate::cmd) use item_module::{HAddModuleCmd, HChangeModuleCmd};
pub(in crate::cmd) use item_ship::{
    HChangeShipCmd, HChangeShipViaFitIdCmd, HChangeShipViaItemIdCmd, HSetShipCmd, HUnsetShipCmd,
};
pub(in crate::cmd) use item_stance::{
    HChangeStanceCmd, HChangeStanceViaFitIdCmd, HChangeStanceViaItemIdCmd, HSetStanceCmd, HUnsetStanceCmd,
};
pub(crate) use main::HChangeFitCommand;

mod fit;
mod item_autocharge;
mod item_character;
mod item_charge;
mod item_fighter;
mod item_module;
mod item_ship;
mod item_stance;
mod main;
