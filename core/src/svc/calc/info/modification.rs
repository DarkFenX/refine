use crate::{
    api::Op,
    num::{PValue, Value},
    svc::calc::Affector,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Modification {
    pub op: Op,
    pub initial_str: Value,
    pub range_mult: Option<PValue>,
    pub resist_mult: Option<PValue>,
    pub stacking_mult: Option<PValue>,
    pub applied_str: Value,
    pub affectors: Vec<Affector>,
}
