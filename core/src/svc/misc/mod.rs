pub(crate) use ctx::SvcCtx;
pub(crate) use effect_proj::get_proj_mult;
pub(crate) use effect_resist::get_resist_mult;
pub(in crate::svc) use effect_resist::{get_resist_a_attr_id, get_resist_mult_val_by_projectee_aspec};
pub(in crate::svc) use spool::{ResolvedSpool, resolve_spool};

mod ctx;
mod effect_proj;
mod effect_resist;
mod spool;
