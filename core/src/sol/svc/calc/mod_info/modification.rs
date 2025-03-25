use crate::sol::{AttrVal, OpInfo, svc::calc::AffectorInfo};

pub struct ModificationInfo {
    pub op: OpInfo,
    pub initial_val: AttrVal,
    pub range_mult: Option<AttrVal>,
    pub resist_mult: Option<AttrVal>,
    pub stacking_mult: Option<AttrVal>,
    pub applied_val: AttrVal,
    pub affectors: Vec<AffectorInfo>,
}
