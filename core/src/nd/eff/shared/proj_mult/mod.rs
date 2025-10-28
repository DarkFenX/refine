pub(in crate::nd::eff) use composite::{
    get_aoe_burst_proj_mult, get_bomb_proj_mult, get_breacher_proj_mult, get_bubble_proj_mult, get_dd_lance_proj_mult,
    get_dd_neut_proj_mult, get_disintegrator_proj_mult, get_guided_bomb_proj_mult, get_missile_proj_mult,
    get_neut_proj_mult, get_noapp_bomb_proj_mult, get_noapp_full_proj_mult, get_noapp_simple_c2s_proj_mult,
    get_noapp_simple_s2s_proj_mult, get_null_proj_mult, get_turret_proj_mult, get_vorton_proj_mult,
};
pub(in crate::nd::eff) use modification::{
    get_aoe_burst_mod_proj_attrs, get_full_mod_proj_attrs, get_simple_mod_proj_attrs,
};

mod application;
mod composite;
mod modification;
mod range;
