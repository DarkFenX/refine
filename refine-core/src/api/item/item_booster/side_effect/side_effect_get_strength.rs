use crate::{Op, SideEffect, Value};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Strength of a side effect, as an operator and the resolved strength value.
pub struct SideEffectStr {
    op: Op,
    strength: Value,
}
impl SideEffectStr {
    pub fn get_op(&self) -> Op {
        self.op
    }
    pub fn get_strength(&self) -> Value {
        self.strength
    }
}

impl<'s> SideEffect<'s> {
    /// Get side effect strength as an operator and modification value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and almost the
    /// same strength.
    pub fn get_strength(&mut self) -> Option<SideEffectStr> {
        let effect_rid = self.sol.u_data.r_data.get_effect_rid_by_aid(&self.effect_aid).unwrap();
        let (op, strength) = self
            .sol
            .svc
            .get_effect_mods_op_str(&self.sol.u_data, self.item_uid, effect_rid)?;
        Some(SideEffectStr {
            op: Op::from_calc_op(op),
            strength,
        })
    }
}
