pub use fit_iter::ModuleIter;
pub use get::GetModuleError;
pub use module::{Module, ModuleMut};
pub use remove_charge::RemoveModuleChargeError;

mod add;
mod fit_iter;
mod get;
mod module;
mod mutation;
mod proj;
mod remove;
mod remove_charge;
mod set_charge;
mod set_state;
mod shared;
