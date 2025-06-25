pub(in crate::sol::svc) use attr_spec::AttrSpec;
pub(in crate::sol::svc) use ctx::SvcCtx;
pub(in crate::sol::svc) use effect_resist::{
    get_resist_a_attr_id, get_resist_mult_val, get_resist_mult_val_by_projectee_aspec,
};
pub(in crate::sol::svc) use effect_spec::EffectSpec;

mod attr_spec;
mod ctx;
mod effect_resist;
mod effect_spec;
