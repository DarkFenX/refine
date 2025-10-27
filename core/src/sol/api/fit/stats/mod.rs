pub use err::{FitCharacterStatError, FitShipStatError};
pub use stat_dmg::FitStatDmgAppliedError;

mod err;
mod shared;
mod stat_char;
mod stat_dmg;
mod stat_remote_nps;
mod stat_remote_rps;
mod stat_resource;
mod stat_ship_physics;
mod stat_ship_sensors;
mod stat_ship_tank;
mod stat_slots;
