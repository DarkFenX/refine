use itertools::Itertools;

use crate::{
    ad::AEffectId,
    api::{AttrId, FullSideEffect, FullSideEffectMut, Op},
    num::Value,
    rd::{REffectModStrength, Src},
};

pub struct SideEffectPartialStr {
    op: Op,
    attr_id: Option<AttrId>,
    // Used only to generate full side effect strength with modification value
    strength: REffectModStrength,
}
impl SideEffectPartialStr {
    pub fn get_op(&self) -> Op {
        self.op
    }
    pub fn get_attr_id(&self) -> Option<AttrId> {
        self.attr_id
    }
}

pub struct SideEffectStr {
    op: Op,
    attr_id: Option<AttrId>,
    strength: Value,
}
impl SideEffectStr {
    pub fn get_op(&self) -> Op {
        self.op
    }
    pub fn get_attr_id(&self) -> Option<AttrId> {
        self.attr_id
    }
    pub fn get_strength(&self) -> Value {
        self.strength
    }
}

impl<'a> FullSideEffect<'a> {
    /// Get side effect strength as an operator and attribute ID which contains strength value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength_partial(&self) -> Option<SideEffectPartialStr> {
        get_strength_partial(&self.sol.u_data.src, &self.effect_aid)
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Get side effect strength as an operator and attribute ID which contains strength value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength_partial(&self) -> Option<SideEffectPartialStr> {
        get_strength_partial(&self.sol.u_data.src, &self.effect_aid)
    }
    /// Get side effect strength as an operator and modification value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength(&mut self) -> Option<SideEffectStr> {
        let partial = self.get_strength_partial()?;
        let strength = match partial.strength {
            REffectModStrength::Attr(attr_rid) => match self.sol.internal_get_item_attr(self.item_uid, attr_rid) {
                Ok(calc_val) => calc_val.extra,
                Err(_) => return None,
            },
            REffectModStrength::Hardcoded(strength) => strength,
        };
        Some(SideEffectStr {
            op: partial.op,
            attr_id: partial.attr_id,
            strength,
        })
    }
}

fn get_strength_partial(src: &Src, effect_id: &AEffectId) -> Option<SideEffectPartialStr> {
    let effect_rid = src.get_effect_rid_by_aid(effect_id).unwrap();
    let mut se_strs = src
        .get_effect_by_rid(effect_rid)
        .modifiers
        .iter()
        .map(|modifier| (modifier.op, modifier.strength))
        .collect_vec();
    match se_strs.len() {
        0 => None,
        1 => se_strs
            .into_iter()
            .map(|(op, strength)| SideEffectPartialStr {
                op: Op::from_a_op(op),
                attr_id: strength
                    .get_attr_rid()
                    .map(|v| AttrId::from_aid(src.get_attr_by_rid(v).aid)),
                strength,
            })
            .next(),
        _ => {
            let (base_op, base_strength) = se_strs.pop().unwrap();
            match se_strs
                .into_iter()
                .all(|(op, strength)| op == base_op && strength == base_strength)
            {
                true => Some(SideEffectPartialStr {
                    op: Op::from_a_op(base_op),
                    attr_id: base_strength
                        .get_attr_rid()
                        .map(|v| AttrId::from_aid(src.get_attr_by_rid(v).aid)),
                    strength: base_strength,
                }),
                false => None,
            }
        }
    }
}
