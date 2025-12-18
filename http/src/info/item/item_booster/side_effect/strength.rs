use super::HSideEffectOp;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HSideEffectStr {
    op: HSideEffectOp,
    val: rc::AttrVal,
}
impl TryFrom<rc::SideEffectStr> for HSideEffectStr {
    type Error = HSideEffectStrConvError;
    fn try_from(core_side_effect_strength: rc::SideEffectStr) -> Result<Self, Self::Error> {
        let raw_val = core_side_effect_strength.get_value();
        match core_side_effect_strength.get_op() {
            rc::Op::Add => Ok(HSideEffectStr {
                op: HSideEffectOp::Add,
                val: raw_val,
            }),
            rc::Op::Sub => Ok(HSideEffectStr {
                op: HSideEffectOp::Add,
                val: -raw_val,
            }),
            rc::Op::PreMul | rc::Op::PostMul | rc::Op::ExtraMul => Ok(HSideEffectStr {
                op: HSideEffectOp::Perc,
                val: (raw_val - rc::AttrVal::from(1.0)) * rc::AttrVal::from(100.0),
            }),
            rc::Op::PreDiv | rc::Op::PostDiv => match raw_val.into_inner() {
                0.0 => Err(HSideEffectStrConvError {}),
                _ => Ok(HSideEffectStr {
                    op: HSideEffectOp::Perc,
                    val: (rc::AttrVal::from(1.0) / raw_val - rc::AttrVal::from(1.0)) * rc::AttrVal::from(100.0),
                }),
            },
            rc::Op::PostPerc => Ok(HSideEffectStr {
                op: HSideEffectOp::Perc,
                val: raw_val,
            }),
            _ => Err(HSideEffectStrConvError {}),
        }
    }
}

pub(crate) struct HSideEffectStrConvError {}
