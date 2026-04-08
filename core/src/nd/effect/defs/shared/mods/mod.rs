pub(in crate::nd::effect::defs) use cloak::{mk_can_cloak_mod, mk_cannot_cloak_mod, mk_disallow_cloak_buff};
pub(in crate::nd::effect::defs) use damp::add_damp_mods;
pub(in crate::nd::effect::defs) use dd::make_dd_self_debuffs;
pub(in crate::nd::effect::defs) use prop::{mk_mjd_sig_mod, mk_mwd_sig_mod, mk_prop_mass_mod};
pub(in crate::nd::effect::defs) use subsystem::mk_subsystem_mod;
pub(in crate::nd::effect::defs) use tp::add_tp_mods;
pub(in crate::nd::effect::defs) use wd::{add_gd_mods, add_td_mods, add_wd_mods};
pub(in crate::nd::effect::defs) use web::add_web_mods;

mod cloak;
mod damp;
mod dd;
mod prop;
mod subsystem;
mod tp;
mod wd;
mod web;
