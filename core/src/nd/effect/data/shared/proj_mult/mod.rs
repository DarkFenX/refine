pub(in crate::nd::effect::data) use application::get_radius_ratio_mult;
pub(in crate::nd::effect::data) use composite::{
    get_aoe_burst_noapp_proj_mult, get_aoe_burst_proj_mult, get_aoe_dd_dmg_proj_mult, get_aoe_dd_noapp_proj_mult,
    get_aoe_dd_side_neut_proj_mult, get_bomb_noapp_proj_mult, get_breacher_proj_mult, get_bubble_proj_mult,
    get_disintegrator_proj_mult, get_full_noapp_proj_mult, get_guided_bomb_proj_mult, get_missile_proj_mult,
    get_neut_proj_mult, get_null_proj_mult, get_simple_s2s_noapp_proj_mult, get_turret_proj_mult, get_vorton_proj_mult,
};
pub(in crate::nd::effect::data) use modification::{
    get_aoe_burst_mod_proj_attrs, get_aoe_dd_mod_proj_attrs, get_full_mod_proj_attrs, get_simple_mod_proj_attrs,
};
pub(in crate::nd::effect::data) use range::get_bomb_range_mult;

mod application;
mod composite;
mod modification;
mod range;
