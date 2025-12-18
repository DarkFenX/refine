use super::{affector::HAffector, op::HModOp};

#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::item::extended) struct HModification {
    op: HModOp,
    initial_val: rc::AttrVal,
    range_mult: Option<rc::AttrVal>,
    resist_mult: Option<rc::AttrVal>,
    stacking_mult: Option<rc::AttrVal>,
    applied_val: rc::AttrVal,
    src: Vec<HAffector>,
}
impl From<&rc::Modification> for HModification {
    fn from(core_modification: &rc::Modification) -> Self {
        Self {
            op: (&core_modification.op).into(),
            initial_val: core_modification.initial_val,
            range_mult: core_modification.range_mult,
            resist_mult: core_modification.resist_mult,
            stacking_mult: core_modification.stacking_mult,
            applied_val: core_modification.applied_val,
            src: core_modification.affectors.iter().map(Into::into).collect(),
        }
    }
}
