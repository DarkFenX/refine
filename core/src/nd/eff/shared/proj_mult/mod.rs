pub(in crate::nd::eff) use aoe_burst::{get_proj_attrs_aoe_burst, get_proj_mult_aoe_burst};
pub(in crate::nd::eff) use missile::{get_bomb_proj_mult, get_missile_proj_mult};
pub(in crate::nd::eff) use normal::{
    get_mod_proj_attrs_normal, get_proj_mult_normal_restricted, get_proj_mult_normal_unrestricted,
};
pub(in crate::nd::eff) use simple::{get_proj_attrs_simple, get_proj_mult_simple_c2s, get_proj_mult_simple_s2s};

mod aoe_burst;
mod missile;
mod normal;
mod shared;
mod simple;
