pub use drone::{Drone, DroneMut};
pub use sol_get_drone::GetDroneError;

mod drone;
mod drone_remove;
mod drone_set_coordinates;
mod drone_set_movement;
mod drone_set_prop_mode;
mod drone_set_state;
mod drone_set_type_id;
mod fit_add_drone;
mod fit_iter_drones;
mod mutation;
mod ranged_proj;
mod sol_get_drone;
mod util_add_remove;
mod util_physics;
