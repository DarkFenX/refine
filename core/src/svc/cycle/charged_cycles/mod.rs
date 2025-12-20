pub(in crate::svc::cycle) use autocharge::get_autocharge_cycle_count;
pub(in crate::svc::cycle) use charge_rate::get_charge_rate_cycle_count;
pub(in crate::svc::cycle) use crystal::get_crystal_cycle_count;
pub(in crate::svc::cycle) use shared::ChargedCycleCount;
pub(in crate::svc::cycle) use uncharged::get_uncharged_cycle_count;
pub(in crate::svc::cycle) use undepletable::get_undepletable_cycle_count;

mod autocharge;
mod charge_rate;
mod crystal;
mod shared;
mod uncharged;
mod undepletable;
