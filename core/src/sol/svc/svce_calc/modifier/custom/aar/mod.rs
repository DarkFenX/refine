pub(in crate::sol::svc::svce_calc::modifier) use affector_attr::AAR_AFFECTOR_ATTR_ID;
pub(in crate::sol::svc::svce_calc::modifier) use mod_val_getter::get_mod_val;
pub(in crate::sol::svc::svce_calc) use modifier::make_mod;
pub(in crate::sol::svc::svce_calc::modifier) use revision::revise_on_item_add_removal;

mod affector_attr;
mod mod_val_getter;
mod modifier;
mod revision;
