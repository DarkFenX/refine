use super::{op::HModOp, src::HModSrcInfo};

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HModificationInfo {
    pub(crate) val: rc::AttrVal,
    pub(crate) op: HModOp,
    pub(crate) penalized: bool,
    pub(crate) src: Vec<HModSrcInfo>,
}
impl HModificationInfo {
    fn new(val: rc::AttrVal, op: HModOp, penalized: bool, src: Vec<HModSrcInfo>) -> Self {
        Self {
            val,
            op,
            penalized,
            src,
        }
    }
}
impl From<&rc::SsModInfo> for HModificationInfo {
    fn from(core_mod_info: &rc::SsModInfo) -> Self {
        Self::new(
            core_mod_info.val,
            (&core_mod_info.op).into(),
            core_mod_info.penalized,
            core_mod_info.src.iter().map(|v| v.into()).collect(),
        )
    }
}
