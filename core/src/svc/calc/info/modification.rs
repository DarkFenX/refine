use crate::{
    api::Op,
    num::{PValue, Value},
    svc::calc::Affector,
};

pub struct Modification {
    pub op: Op,
    pub initial_val: Value,
    pub range_mult: Option<PValue>,
    pub resist_mult: Option<PValue>,
    pub stacking_mult: Option<PValue>,
    pub applied_val: Value,
    pub affectors: Vec<Affector>,
}
