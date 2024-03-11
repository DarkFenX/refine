use super::{aggr_mode::HModAggrMode, op::HModOp, src::HModSrcInfo};

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HModificationInfo {
    pub(crate) val: rc::AttrVal,
    pub(crate) op: HModOp,
    pub(crate) penalized: bool,
    pub(crate) aggr_mode: HModAggrMode,
    pub(crate) src: Vec<HModSrcInfo>,
}
impl HModificationInfo {
    fn new(
        val: rc::AttrVal,
        op: HModOp,
        penalized: bool,
        aggr_mode: HModAggrMode,
        src: Vec<HModSrcInfo>,
    ) -> Self {
        Self {
            val,
            op,
            penalized,
            aggr_mode,
            src,
        }
    }
}
impl From<&rc::ModInfo> for HModificationInfo {
    fn from(core_mod_info: &rc::ModInfo) -> Self {
        Self::new(
            core_mod_info.val,
            (&core_mod_info.op).into(),
            core_mod_info.penalized,
            (&core_mod_info.aggr_mode).into(),
            core_mod_info.src.iter().map(|v| v.into()).collect()
        )
    }
}
