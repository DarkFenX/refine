pub(in crate::nd::eff) use dmg_dd::{get_aoe_dd_dmg_opc, get_direct_dd_dmg_opc};
pub(in crate::nd::eff) use dmg_missile::get_missile_dmg_opc;
pub(in crate::nd::eff) use mining::{get_mining_opc, get_mining_values};
pub(in crate::nd::eff) use neut::{get_aoe_dd_side_neut_opc, get_generic_neut_opc};
pub(in crate::nd::eff) use rep::{
    get_local_armor_rep_opc, get_local_hull_rep_opc, get_local_shield_rep_opc, get_outgoing_armor_rep_opc,
    get_outgoing_cap_rep_opc, get_outgoing_hull_rep_opc, get_outgoing_shield_rep_opc,
};

mod dmg_dd;
mod dmg_missile;
mod mining;
mod neut;
mod rep;
