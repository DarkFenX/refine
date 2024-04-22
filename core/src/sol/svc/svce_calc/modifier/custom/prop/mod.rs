pub(in crate::sol::svc::svce_calc) use attr_mod::make_mod;
pub(in crate::sol::svc::svce_calc::modifier) use deps::on_effect_stop;
pub(in crate::sol::svc::svce_calc::modifier) use mod_src::get_srcs;
pub(in crate::sol::svc::svce_calc::modifier) use mod_val_getter::get_mod_val;

mod attr;
mod attr_mod;
mod deps;
mod misc;
mod mod_src;
mod mod_val_getter;
