pub use add_fit::FleetAddFitError;
pub use fleet::{Fleet, FleetMut};
pub use get::GetFleetError;
pub use remove_fit::FleetRemoveFitError;

mod add;
mod add_fit;
mod fleet;
mod get;
mod iter_fits;
mod remove;
mod remove_fit;
mod sol_iter;
