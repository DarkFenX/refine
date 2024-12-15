//! Solar system extension methods which handle fit manipulation.

pub use sole_get_fit::GetFitError;
pub use sole_remove_fit::RemoveFitError;
pub use sole_set_fit_fleet::SetFitFleetError;
pub use sole_unset_fit_fleet::UnsetFitFleetError;

mod sole_add_fit;
mod sole_get_fit;
mod sole_get_fits;
mod sole_remove_fit;
mod sole_set_fit_fleet;
mod sole_unset_fit_fleet;
