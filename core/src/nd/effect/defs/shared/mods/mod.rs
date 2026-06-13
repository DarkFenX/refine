pub(in crate::nd::effect::defs) use cloak::{
    mk_cannot_cloak_mod_hardcoded, mk_cannot_cloak_mod_transfer, mk_disallow_cloak_buff,
};
pub(in crate::nd::effect::defs) use damp::add_damp_mods;
pub(in crate::nd::effect::defs) use dd::{make_burst_proj_self_mods, make_dd_self_debuffs};
pub(in crate::nd::effect::defs) use misc::mk_disallow_assistance_mod_transfer;
pub(in crate::nd::effect::defs) use prop::{mk_mjd_mods, mk_mwd_sig_mod, mk_prop_mass_mod};
pub(in crate::nd::effect::defs) use subsystem::mk_subsystem_mod;
pub(in crate::nd::effect::defs) use tackle::{add_web_mods, mk_bubble_buff};
pub(in crate::nd::effect::defs) use tp::add_tp_mods;
pub(in crate::nd::effect::defs) use wd::{add_gd_mods, add_td_mods, add_wd_mods};

mod cloak;
mod damp;
mod dd;
mod misc;
mod prop;
mod subsystem;
mod tackle;
mod tp;
mod wd;
