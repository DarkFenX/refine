pub use fit_iter_modules::ModuleIter;
pub use module::{Module, ModuleMut};
pub use sol_get_module::GetModuleError;

mod fit_add_module;
mod fit_iter_modules;
mod module;
mod module_remove;
mod module_set_charge;
mod module_set_state;
mod module_set_type_id;
mod mutation;
mod ranged_proj;
mod shared;
mod sol_get_module;
mod util_add_remove;
