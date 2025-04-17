use super::HSideEffectOp;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HSideEffectStr {
    pub(crate) op: HSideEffectOp,
    pub(crate) val: rc::AttrVal,
}
impl TryFrom<rc::SideEffectStr> for HSideEffectStr {
    type Error = HSideEffectStrConvError;
    fn try_from(core_side_effect_strength: rc::SideEffectStr) -> Result<Self, Self::Error> {
        let raw_val = core_side_effect_strength.get_value();
        match core_side_effect_strength.get_op() {
            rc::OpInfo::Add => Ok(HSideEffectStr {
                op: HSideEffectOp::Add,
                val: raw_val,
            }),
            rc::OpInfo::Sub => Ok(HSideEffectStr {
                op: HSideEffectOp::Add,
                val: -raw_val,
            }),
            rc::OpInfo::PreMul | rc::OpInfo::PostMul | rc::OpInfo::ExtraMul => Ok(HSideEffectStr {
                op: HSideEffectOp::Perc,
                val: (raw_val - rc::AttrVal::from(1.0)) * rc::AttrVal::from(100.0),
            }),
            rc::OpInfo::PreDiv | rc::OpInfo::PostDiv => match raw_val.into_inner() {
                0.0 => Err(HSideEffectStrConvError {}),
                _ => Ok(HSideEffectStr {
                    op: HSideEffectOp::Perc,
                    val: (rc::AttrVal::from(1.0) / raw_val - rc::AttrVal::from(1.0)) * rc::AttrVal::from(100.0),
                }),
            },
            rc::OpInfo::PostPerc => Ok(HSideEffectStr {
                op: HSideEffectOp::Perc,
                val: raw_val,
            }),
            _ => Err(HSideEffectStrConvError {}),
        }
    }
}

struct HSideEffectStrConvError {}
