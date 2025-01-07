use crate::{
    defs::AttrVal,
    sol::svc::calc::{SolAffectorInfo, SolOpInfo},
};

pub struct SolModificationInfo {
    pub op: SolOpInfo,
    pub initial_val: AttrVal,
    pub range_mult: Option<AttrVal>,
    pub resist_mult: Option<AttrVal>,
    pub stacking_mult: Option<AttrVal>,
    pub applied_val: AttrVal,
    pub affectors: Vec<SolAffectorInfo>,
}
impl SolModificationInfo {
    pub(in crate::sol::svc::calc) fn new(
        op: SolOpInfo,
        initial_val: AttrVal,
        range_mult: Option<AttrVal>,
        resist_mult: Option<AttrVal>,
        stacking_mult: Option<AttrVal>,
        applied_val: AttrVal,
        affectors: Vec<SolAffectorInfo>,
    ) -> Self {
        Self {
            op,
            initial_val,
            range_mult,
            resist_mult,
            stacking_mult,
            applied_val,
            affectors,
        }
    }
}
