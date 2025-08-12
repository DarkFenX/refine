pub use fleet::{Fleet, FleetMut};
pub use fleet_add_fit::FleetAddFitError;
pub use fleet_remove_fit::FleetRemoveFitError;
pub use sol_get_fleet::GetFleetError;

mod fleet;
mod fleet_add_fit;
mod fleet_iter_fits;
mod fleet_remove;
mod fleet_remove_fit;
mod sol_add_fleet;
mod sol_get_fleet;
mod sol_iter_fleets;
mod stats;
