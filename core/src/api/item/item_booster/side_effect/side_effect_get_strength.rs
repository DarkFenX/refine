use itertools::Itertools;

use crate::{
    ad::AEffectId,
    api::{AttrId, FullSideEffect, FullSideEffectMut, Op},
    misc::Value,
    rd::{RAttrId, Src},
};

pub struct SideEffectPartialStr {
    op: Op,
    attr_id: AttrId,
    // Used only to generate full side effect strength with modification value
    attr_rid: RAttrId,
}
impl SideEffectPartialStr {
    pub fn get_op(&self) -> Op {
        self.op
    }
    pub fn get_attr_id(&self) -> AttrId {
        self.attr_id
    }
}

pub struct SideEffectStr {
    op: Op,
    attr_id: AttrId,
    value: Value,
}
impl SideEffectStr {
    pub fn get_op(&self) -> Op {
        self.op
    }
    pub fn get_attr_id(&self) -> AttrId {
        self.attr_id
    }
    pub fn get_value(&self) -> Value {
        self.value
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
        match self.get_strength_partial() {
            Some(partial) => match self.sol.internal_get_item_attr(self.item_uid, partial.attr_rid) {
                Ok(calc_val) => Some(SideEffectStr {
                    op: partial.op,
                    attr_id: partial.attr_id,
                    value: calc_val.extra,
                }),
                Err(_) => None,
            },
            None => None,
        }
    }
}

fn get_strength_partial(src: &Src, effect_id: &AEffectId) -> Option<SideEffectPartialStr> {
    let effect_rid = src.get_effect_rid_by_aid(effect_id).unwrap();
    let mut se_strs = src
        .get_effect_by_rid(effect_rid)
        .modifiers
        .iter()
        .map(|modifier| (modifier.op, modifier.affector_attr_rid))
        .collect_vec();
    match se_strs.len() {
        0 => None,
        1 => se_strs
            .into_iter()
            .map(|(a_op, attr_rid)| SideEffectPartialStr {
                op: Op::from_a_op(a_op),
                attr_id: AttrId::from_aid(src.get_attr_by_rid(attr_rid).aid),
                attr_rid,
            })
            .next(),
        _ => {
            let (base_op, base_attr_rid) = se_strs.pop().unwrap();
            match se_strs
                .into_iter()
                .all(|(op, attr_rid)| op == base_op && attr_rid == base_attr_rid)
            {
                true => Some(SideEffectPartialStr {
                    op: Op::from_a_op(base_op),
                    attr_id: AttrId::from_aid(src.get_attr_by_rid(base_attr_rid).aid),
                    attr_rid: base_attr_rid,
                }),
                false => None,
            }
        }
    }
}
