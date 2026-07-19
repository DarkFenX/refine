pub(crate) use dev::dev_check_sol;
pub(crate) use fit::{add_fit, get_fit, remove_fit};
pub(crate) use root::root;
pub(crate) use sol::{add_sol, change_sol, get_sol, remove_sol};
pub(crate) use src::{add_source, remove_source};

mod dev;
mod fit;
mod root;
mod sol;
mod src;
