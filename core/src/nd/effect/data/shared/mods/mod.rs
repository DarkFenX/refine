pub(in crate::nd::effect::data) use damp::add_damp_mods;
pub(in crate::nd::effect::data) use dd::make_dd_self_debuffs;
pub(in crate::nd::effect::data) use prop::{add_prop_speed_mod, mk_mjd_sig_mod, mk_mwd_sig_mod, mk_prop_mass_mod};
pub(in crate::nd::effect::data) use subsystem::mk_subsystem_mod;
pub(in crate::nd::effect::data) use tp::add_tp_mods;
pub(in crate::nd::effect::data) use wd::{add_gd_mods, add_td_mods, add_wd_mods};
pub(in crate::nd::effect::data) use web::add_web_mods;

mod damp;
mod dd;
mod prop;
mod subsystem;
mod tp;
mod wd;
mod web;
