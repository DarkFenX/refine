pub(crate) use custom::CustomAffectorValue;
pub(in crate::sol::svc::calc) use extend::extend_with_custom_mods;

mod custom;
mod extend;
pub(super) mod missile_flight_time;
pub(super) mod shared;
