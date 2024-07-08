use crate::{
    defs::AttrVal,
    sol::svc::svce_calc::{SolAffectorInfo, SolOpInfo},
};

pub struct SolModificationInfo {
    pub initial_val: AttrVal,
    pub range_mult: Option<AttrVal>,
    pub resist_mult: Option<AttrVal>,
    pub intermediate_val: AttrVal,
    pub stacking_mult: Option<AttrVal>,
    pub applied_val: Option<AttrVal>,
    pub op: SolOpInfo,
    pub affectors: Vec<SolAffectorInfo>,
}
impl SolModificationInfo {
    pub(in crate::sol::svc::svce_calc) fn new(
        initial_val: AttrVal,
        range_mult: Option<AttrVal>,
        resist_mult: Option<AttrVal>,
        intermediate_val: AttrVal,
        stacking_mult: Option<AttrVal>,
        applied_val: Option<AttrVal>,
        op: SolOpInfo,
        affectors: Vec<SolAffectorInfo>,
    ) -> Self {
        Self {
            initial_val,
            range_mult,
            resist_mult,
            intermediate_val,
            stacking_mult,
            applied_val,
            op,
            affectors,
        }
    }
}
