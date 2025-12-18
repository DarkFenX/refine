use crate::{api::Op, def::AttrVal, svc::calc::Affector};

pub struct Modification {
    pub op: Op,
    pub initial_val: AttrVal,
    pub range_mult: Option<AttrVal>,
    pub resist_mult: Option<AttrVal>,
    pub stacking_mult: Option<AttrVal>,
    pub applied_val: AttrVal,
    pub affectors: Vec<Affector>,
}
