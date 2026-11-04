use crate::{Op, UnitInterval, Value};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct SideEffectInfo {
    pub chance: UnitInterval,
    pub state: bool,
    pub modification: Option<SideEffectMod>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct SideEffectMod {
    pub op: SideEffectOp,
    pub str: Value,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum SideEffectOp {
    Add,
    Perc,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SideEffectInfo {
    pub(in crate::info::item) fn from_core(mut core_side_effect: rc::SideEffect) -> Self {
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
            Op::Add | Op::ExtraAdd => Some(Self {
                op: SideEffectOp::Add,
                str: raw_strength,
            }),
            Op::Sub => Some(Self {
                op: SideEffectOp::Add,
                str: Value::from_f64_clamped(-raw_strength.into_f64()),
            }),
            Op::PreMul | Op::PostMul | Op::ExtraMul => Some(Self {
                op: SideEffectOp::Perc,
                str: Value::from_f64_clamped(raw_strength.into_f64().mul_add(100.0, -100.0)),
            }),
            Op::PreDiv | Op::PostDiv => match raw_strength.into_f64() {
                0.0 => None,
                v => Some(Self {
                    op: SideEffectOp::Perc,
                    str: Value::from_f64_clamped(100.0 / v - 100.0),
                }),
            },
            Op::PostPerc => Some(Self {
                op: SideEffectOp::Perc,
                str: raw_strength,
            }),
            Op::BaseAssign | Op::PreAssign | Op::PostAssign | Op::MinLimit | Op::MaxLimit => None,
        }
    }
}
