use super::{op::HModOp, src::HModSrcInfo};

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HModificationInfo {
    pub(crate) initial_val: rc::AttrVal,
    pub(crate) range_reduction: Option<rc::AttrVal>,
    pub(crate) resist_reduction: Option<rc::AttrVal>,
    pub(crate) intermediate_val: rc::AttrVal,
    pub(crate) stacking_reduction: Option<rc::AttrVal>,
    pub(crate) applied_val: rc::AttrVal,
    pub(crate) op: HModOp,
    pub(crate) src: Vec<HModSrcInfo>,
}
impl HModificationInfo {
    fn new(
        initial_val: rc::AttrVal,
        range_reduction: Option<rc::AttrVal>,
        resist_reduction: Option<rc::AttrVal>,
        intermediate_val: rc::AttrVal,
        stacking_reduction: Option<rc::AttrVal>,
        applied_val: rc::AttrVal,
        op: HModOp,
        src: Vec<HModSrcInfo>,
    ) -> Self {
        Self {
            initial_val,
            range_reduction,
            resist_reduction,
            intermediate_val,
            stacking_reduction,
            applied_val,
            op,
            src,
        }
    }
}
impl From<&rc::SolModificationInfo> for HModificationInfo {
    fn from(core_mod_info: &rc::SolModificationInfo) -> Self {
        Self::new(
            core_mod_info.initial_val,
            core_mod_info.range_reduction,
            core_mod_info.resist_reduction,
            core_mod_info.intermediate_val,
            core_mod_info.stacking_reduction,
            core_mod_info.applied_val,
            (&core_mod_info.op).into(),
            core_mod_info.affectors.iter().map(|v| v.into()).collect(),
        )
    }
}
