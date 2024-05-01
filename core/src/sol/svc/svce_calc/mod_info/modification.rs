use crate::{
    defs::AttrVal,
    sol::svc::svce_calc::{SolAffectorInfo, SolOpInfo},
};

pub struct SolModificationInfo {
    pub val: AttrVal,
    pub op: SolOpInfo,
    pub penalized: bool,
    pub affectors: Vec<SolAffectorInfo>,
}
impl SolModificationInfo {
    pub(in crate::sol::svc::svce_calc) fn new(
        val: AttrVal,
        op: SolOpInfo,
        penalized: bool,
        affectors: Vec<SolAffectorInfo>,
    ) -> Self {
        Self {
            val,
            op,
            penalized,
            affectors,
        }
    }
}
