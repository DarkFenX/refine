pub use fleet::{Fleet, FleetMut};
pub use fleet_add_fit::FleetFitAddError;
pub use fleet_remove_fit::FleetFitRemoveError;
pub use sol_get_fleet::FleetGetError;
pub use stats::StatFleetAppliedError;

mod fleet;
mod fleet_add_fit;
mod fleet_iter_fits;
mod fleet_remove;
mod fleet_remove_fit;
mod sol_add_fleet;
mod sol_get_fleet;
mod sol_iter_fleets;
mod stats;
