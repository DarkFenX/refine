use crate::{Op, SideEffect, Value, rd::REffectModStrength};

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
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength(&mut self) -> Option<SideEffectStr> {
        let r_data = &self.core.u_data.r_data;
        let effect_rid = r_data.get_effect_rid_by_aid(&self.effect_aid).unwrap();
        let mut se_strs = r_data
            .get_effect_by_rid(effect_rid)
            .modifiers
            .iter()
            .map(|modifier| (modifier.op, modifier.strength));
        let (op, strength) = se_strs.next()?;
        if !se_strs.all(|se_str| se_str == (op, strength)) {
            return None;
        }
        let strength = match strength {
            REffectModStrength::Attr(attr_rid) => match self.core.internal_get_item_attr(self.item_uid, attr_rid) {
                Ok(calc_val) => calc_val.extra,
                Err(..) => return None,
            },
            REffectModStrength::Hardcoded(strength) => strength,
        };
        Some(SideEffectStr {
            op: Op::from_a_op(op),
            strength,
        })
    }
}
