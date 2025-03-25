use super::{affector::HAffectorInfo, op::HModOp};

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HModificationInfo {
    pub(crate) op: HModOp,
    pub(crate) initial_val: rc::AttrVal,
    pub(crate) range_mult: Option<rc::AttrVal>,
    pub(crate) resist_mult: Option<rc::AttrVal>,
    pub(crate) stacking_mult: Option<rc::AttrVal>,
    pub(crate) applied_val: rc::AttrVal,
    pub(crate) src: Vec<HAffectorInfo>,
}
impl From<&rc::ModificationInfo> for HModificationInfo {
    fn from(core_mod_info: &rc::ModificationInfo) -> Self {
        Self {
            op: (&core_mod_info.op).into(),
            initial_val: core_mod_info.initial_val,
            range_mult: core_mod_info.range_mult,
            resist_mult: core_mod_info.resist_mult,
            stacking_mult: core_mod_info.stacking_mult,
            applied_val: core_mod_info.applied_val,
            src: core_mod_info.affectors.iter().map(|v| v.into()).collect(),
        }
    }
}
