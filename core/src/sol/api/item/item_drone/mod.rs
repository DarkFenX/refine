pub use drone::{Drone, DroneMut};
pub use sol_get_drone::GetDroneError;

mod drone;
mod drone_remove;
mod drone_set_state;
mod drone_set_type_id;
mod fit_add_drone;
mod fit_iter_drones;
mod int_load_unload;
mod mutation;
mod ranged_proj;
mod sol_get_drone;
