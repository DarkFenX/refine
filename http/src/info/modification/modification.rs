use super::{op::HModOp, src::HModSrcInfo};

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HModificationInfo {
    pub(crate) op: HModOp,
    pub(crate) initial_val: rc::AttrVal,
    pub(crate) range_mult: Option<rc::AttrVal>,
    pub(crate) resist_mult: Option<rc::AttrVal>,
    pub(crate) stacking_mult: Option<rc::AttrVal>,
    pub(crate) applied_val: rc::AttrVal,
    pub(crate) src: Vec<HModSrcInfo>,
}
impl HModificationInfo {
    fn new(
        op: HModOp,
        initial_val: rc::AttrVal,
        range_mult: Option<rc::AttrVal>,
        resist_mult: Option<rc::AttrVal>,
        stacking_mult: Option<rc::AttrVal>,
        applied_val: rc::AttrVal,
        src: Vec<HModSrcInfo>,
    ) -> Self {
        Self {
            op,
            initial_val,
            range_mult,
            resist_mult,
            stacking_mult,
            applied_val,
            src,
        }
    }
}
impl From<&rc::SolModificationInfo> for HModificationInfo {
    fn from(core_mod_info: &rc::SolModificationInfo) -> Self {
        Self::new(
            (&core_mod_info.op).into(),
            core_mod_info.initial_val,
            core_mod_info.range_mult,
            core_mod_info.resist_mult,
            core_mod_info.stacking_mult,
            core_mod_info.applied_val,
            core_mod_info.affectors.iter().map(|v| v.into()).collect(),
        )
    }
}
