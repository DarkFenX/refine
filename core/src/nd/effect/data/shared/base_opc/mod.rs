pub(in crate::nd::effect::data) use dmg::{
    get_aoe_dd_dmg_opc_spec, get_direct_dd_dmg_opc_spec, get_instant_charge_mult_dmg_base_opc, get_instant_dmg_base_opc,
};
pub(in crate::nd::effect::data) use ecm::{
    get_aoe_ecm_base_opc, get_direct_ecm_base_opc, get_ecm_burst_base_opc, get_ecm_drone_base_opc,
};
pub(in crate::nd::effect::data) use mining::{get_crit_mining_base_opc, get_mining_base_opc};
pub(in crate::nd::effect::data) use neut::{
    get_aoe_dd_side_neut_opc_spec, get_aoe_neut_base_opc, get_neut_base_opc, get_nosf_base_opc,
};
pub(in crate::nd::effect::data) use rep::{
    get_ancillary_armor_mult, get_armor_rep_base_opc, get_cap_trans_base_opc, get_hull_rep_base_opc,
    get_shield_rep_base_opc,
};

mod dmg;
mod ecm;
mod generic;
mod mining;
mod neut;
mod rep;
