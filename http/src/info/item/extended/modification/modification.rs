use serde_tuple::Serialize_tuple;

use super::{affector::HAffector, op::HModOp};

#[derive(Serialize_tuple)]
pub(in crate::info::item::extended) struct HModification {
    op: HModOp,
    initial_val: f64,
    range_mult: Option<f64>,
    resist_mult: Option<f64>,
    stacking_mult: Option<f64>,
    applied_val: f64,
    src: Vec<HAffector>,
}
impl HModification {
    pub(in crate::info::item::extended) fn from_core(core_modification: rc::Modification) -> Self {
        Self {
            op: HModOp::from_core(core_modification.op),
            initial_val: core_modification.initial_val.into_f64(),
            range_mult: core_modification.range_mult.map(|v| v.into_f64()),
            resist_mult: core_modification.resist_mult.map(|v| v.into_f64()),
            stacking_mult: core_modification.stacking_mult.map(|v| v.into_f64()),
            applied_val: core_modification.applied_val.into_f64(),
            src: core_modification
                .affectors
                .into_iter()
                .map(HAffector::from_core)
                .collect(),
        }
    }
}
