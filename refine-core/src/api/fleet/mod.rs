pub use fit::{FleetFitAddError, FleetFitGetError, FleetFitRemoveError};
pub use fleet::Fleet;
pub use sol_get_fleet::FleetGetError;
pub use stats::{FleetStats, StatFleetAppliedError};

mod fit;
mod fleet;
mod fleet_remove;
mod sol_add_fleet;
mod sol_get_fleet;
mod sol_iter_fleets;
mod stats;
