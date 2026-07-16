use crate::{UnitInterval, Value};

pub struct SideEffectInfo {
    pub chance: UnitInterval,
    pub state: bool,
    pub modification: Option<SideEffectMod>,
}

pub struct SideEffectMod {
    pub op: SideEffectOp,
    pub str: Value,
}

pub enum SideEffectOp {
    Add,
    Perc,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SideEffectInfo {
    pub(in crate::info::item) fn from_core(mut core_side_effect: rc::SideEffectMut) -> Self {
        Self {
            chance: core_side_effect.get_chance(),
            state: core_side_effect.get_state(),
            modification: core_side_effect.get_strength().and_then(SideEffectMod::try_from_core),
        }
    }
}

impl SideEffectMod {
    fn try_from_core(core_sid_str: rc::SideEffectStr) -> Option<Self> {
        let raw_strength = core_sid_str.get_strength();
        match core_sid_str.get_op() {
            rc::Op::Add | rc::Op::ExtraAdd => Some(Self {
                op: SideEffectOp::Add,
                str: raw_strength,
            }),
            rc::Op::Sub => Some(Self {
                op: SideEffectOp::Add,
                str: Value::from_f64(-raw_strength.into_f64()),
            }),
            rc::Op::PreMul | rc::Op::PostMul | rc::Op::ExtraMul => Some(Self {
                op: SideEffectOp::Perc,
                str: Value::from_f64(raw_strength.into_f64().mul_add(100.0, -100.0)),
            }),
            rc::Op::PreDiv | rc::Op::PostDiv => match raw_strength.into_f64() {
                0.0 => None,
                v => Some(Self {
                    op: SideEffectOp::Perc,
                    str: Value::from_f64(100.0 / v - 100.0),
                }),
            },
            rc::Op::PostPerc => Some(Self {
                op: SideEffectOp::Perc,
                str: raw_strength,
            }),
            rc::Op::BaseAssign | rc::Op::PreAssign | rc::Op::PostAssign | rc::Op::MinLimit | rc::Op::MaxLimit => None,
        }
    }
}
