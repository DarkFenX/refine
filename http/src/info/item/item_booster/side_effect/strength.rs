use serde_tuple::Serialize_tuple;

use super::op::HSideEffectOp;

#[derive(Serialize_tuple)]
pub(in crate::info::item::item_booster::side_effect) struct HSideEffectStr {
    op: HSideEffectOp,
    val: f64,
}
impl TryFrom<rc::SideEffectStr> for HSideEffectStr {
    type Error = HSideEffectStrConvError;
    fn try_from(core_side_effect_strength: rc::SideEffectStr) -> Result<Self, Self::Error> {
        let raw_val = core_side_effect_strength.get_value();
        match core_side_effect_strength.get_op() {
            rc::Op::Add => Ok(HSideEffectStr {
                op: HSideEffectOp::Add,
                val: raw_val.into_f64(),
            }),
            rc::Op::Sub => Ok(HSideEffectStr {
                op: HSideEffectOp::Add,
                val: -raw_val.into_f64(),
            }),
            rc::Op::PreMul | rc::Op::PostMul | rc::Op::ExtraMul => Ok(HSideEffectStr {
                op: HSideEffectOp::Perc,
                val: (raw_val.into_f64() - 1.0) * 100.0,
            }),
            rc::Op::PreDiv | rc::Op::PostDiv => match raw_val.into_f64() {
                0.0 => Err(HSideEffectStrConvError {}),
                v => Ok(HSideEffectStr {
                    op: HSideEffectOp::Perc,
                    val: (1.0 / v - 1.0) * 100.0,
                }),
            },
            rc::Op::PostPerc => Ok(HSideEffectStr {
                op: HSideEffectOp::Perc,
                val: raw_val.into_f64(),
            }),
            _ => Err(HSideEffectStrConvError {}),
        }
    }
}

pub(crate) struct HSideEffectStrConvError {}
