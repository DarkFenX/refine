pub(in crate::nd::eff) use aoe_burst::{get_proj_attrs_aoe_burst, get_proj_mult_aoe_burst};
pub(in crate::nd::eff) use missile::{get_proj_mult_bomb, get_proj_mult_missile};
pub(in crate::nd::eff) use normal::{
    get_mod_proj_attrs_normal, get_proj_mult_normal_restricted_s2s, get_proj_mult_normal_unrestricted_s2s,
};
pub(in crate::nd::eff) use simple::{get_proj_attrs_simple, get_proj_mult_simple_s2s};

mod aoe_burst;
mod missile;
mod normal;
mod shared;
mod simple;
