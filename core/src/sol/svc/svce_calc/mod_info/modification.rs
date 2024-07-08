use crate::{
    defs::AttrVal,
    sol::svc::svce_calc::{SolAffectorInfo, SolOpInfo},
};

pub struct SolModificationInfo {
    pub initial_val: AttrVal,
    pub range_reduction: Option<AttrVal>,
    pub resist_reduction: Option<AttrVal>,
    pub intermediate_val: AttrVal,
    pub stacking_reduction: Option<AttrVal>,
    pub applied_val: AttrVal,
    pub op: SolOpInfo,
    pub affectors: Vec<SolAffectorInfo>,
}
impl SolModificationInfo {
    pub(in crate::sol::svc::svce_calc) fn new(
        initial_val: AttrVal,
        range_reduction: Option<AttrVal>,
        resist_reduction: Option<AttrVal>,
        intermediate_val: AttrVal,
        stacking_reduction: Option<AttrVal>,
        applied_val: AttrVal,
        op: SolOpInfo,
        affectors: Vec<SolAffectorInfo>,
    ) -> Self {
        Self {
            initial_val,
            range_reduction,
            resist_reduction,
            intermediate_val,
            stacking_reduction,
            applied_val,
            op,
            affectors,
        }
    }
}
