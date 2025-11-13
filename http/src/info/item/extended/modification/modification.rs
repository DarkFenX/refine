use super::{affector::HAffectorInfo, op::HModOp};

#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::item::extended) struct HModificationInfo {
    op: HModOp,
    initial_val: rc::AttrVal,
    range_mult: Option<rc::AttrVal>,
    resist_mult: Option<rc::AttrVal>,
    stacking_mult: Option<rc::AttrVal>,
    applied_val: rc::AttrVal,
    src: Vec<HAffectorInfo>,
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
            src: core_mod_info.affectors.iter().map(Into::into).collect(),
        }
    }
}
