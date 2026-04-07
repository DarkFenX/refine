use serde_tuple::Serialize_tuple;

use super::op::HSideEffectOp;

#[derive(Serialize_tuple)]
pub(in crate::info::item::item_booster::side_effect) struct HSideEffectModification {
    op: HSideEffectOp,
    str: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSideEffectModification {
    pub(super) fn try_from_core(core_side_effect_strength: rc::SideEffectStr) -> Option<Self> {
        let raw_strength = core_side_effect_strength.get_strength();
        match core_side_effect_strength.get_op() {
            rc::Op::Add => Some(HSideEffectModification {
                op: HSideEffectOp::Add,
                str: raw_strength.into_f64(),
            }),
            rc::Op::Sub => Some(HSideEffectModification {
                op: HSideEffectOp::Add,
                str: -raw_strength.into_f64(),
            }),
            rc::Op::PreMul | rc::Op::PostMul | rc::Op::ExtraMul => Some(HSideEffectModification {
                op: HSideEffectOp::Perc,
                str: (raw_strength.into_f64() - 1.0) * 100.0,
            }),
            rc::Op::PreDiv | rc::Op::PostDiv => match raw_strength.into_f64() {
                0.0 => None,
                v => Some(HSideEffectModification {
                    op: HSideEffectOp::Perc,
                    str: (1.0 / v - 1.0) * 100.0,
                }),
            },
            rc::Op::PostPerc => Some(HSideEffectModification {
                op: HSideEffectOp::Perc,
                str: raw_strength.into_f64(),
            }),
            _ => None,
        }
    }
}
