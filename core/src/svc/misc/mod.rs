pub(crate) use ctx::SvcCtx;
pub(in crate::svc) use effect_resist::{
    get_resist_a_attr_id, get_resist_mult_val, get_resist_mult_val_by_projectee_aspec,
};

mod ctx;
mod effect_resist;
