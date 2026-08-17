#[cfg(feature = "serde")]
pub use id::ParseSolarSystemIdError;
pub use id::SolarSystemId;
pub use info_args::ValSolInfoArgs;
pub use refine_add_sol::SolAddError;
pub use refine_get_sol::SolGetError;
pub use sol::SolarSystem;
pub use sol_batch::SolBatchError;
pub use sol_remove::SolRemoveError;
pub use sol_switch_src::SolSwitchSrcError;

mod id;
mod info_args;
mod refine_add_sol;
mod refine_get_sol;
mod sol;
mod sol_batch;
mod sol_change;
mod sol_get_info;
mod sol_remove;
mod sol_switch_src;
mod sol_validate;
