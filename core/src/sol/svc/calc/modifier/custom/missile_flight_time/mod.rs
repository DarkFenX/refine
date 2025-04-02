pub(in crate::sol::svc::calc::modifier) use affector_info::get_affector_info;
pub(in crate::sol::svc::calc::modifier) use mod_val_getter::get_mod_val;
pub(in crate::sol::svc::calc) use modifier::make_mod;
pub(in crate::sol::svc::calc::modifier) use revision::revise_on_item_add_removal;

mod affector_info;
mod attr;
mod deps;
mod mod_val_getter;
mod modifier;
mod revision;
