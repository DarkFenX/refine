use itertools::Itertools;

use crate::{
    ad::AEffectId,
    api::{AttrId, FullSideEffect, FullSideEffectMut, Op},
    def::AttrVal,
    rd::{RAttrId, Src},
};

pub struct SideEffectPartialStr {
    op: Op,
    attr_id: AttrId,
    // Used only to generate full side effect strength with modification value
    attr_key: RAttrId,
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
    value: AttrVal,
}
impl SideEffectStr {
    pub fn get_op(&self) -> Op {
        self.op
    }
    pub fn get_attr_id(&self) -> AttrId {
        self.attr_id
    }
    pub fn get_value(&self) -> AttrVal {
        self.value
    }
}

impl<'a> FullSideEffect<'a> {
    /// Get side effect strength as an operator and attribute ID which contains strength value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength_partial(&self) -> Option<SideEffectPartialStr> {
        get_strength_partial(&self.sol.u_data.src, &self.effect_id)
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Get side effect strength as an operator and attribute ID which contains strength value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength_partial(&self) -> Option<SideEffectPartialStr> {
        get_strength_partial(&self.sol.u_data.src, &self.effect_id)
    }
    /// Get side effect strength as an operator and modification value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength(&mut self) -> Option<SideEffectStr> {
        match self.get_strength_partial() {
            Some(partial) => match self.sol.internal_get_item_attr(self.key, partial.attr_key) {
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
    let effect_key = src.get_effect_rid_by_aid(effect_id).unwrap();
    let mut se_strs = src
        .get_effect_by_rid(effect_key)
        .modifiers
        .iter()
        .map(|modifier| (modifier.op, modifier.affector_attr_rid))
        .collect_vec();
    match se_strs.len() {
        0 => None,
        1 => se_strs
            .into_iter()
            .map(|(a_op, attr_key)| SideEffectPartialStr {
                op: a_op.into(),
                attr_id: src.get_attr_by_rid(attr_key).aid.into(),
                attr_key,
            })
            .next(),
        _ => {
            let (base_op, base_attr_key) = se_strs.pop().unwrap();
            match se_strs
                .into_iter()
                .all(|(op, attr_key)| op == base_op && attr_key == base_attr_key)
            {
                true => Some(SideEffectPartialStr {
                    op: base_op.into(),
                    attr_id: src.get_attr_by_rid(base_attr_key).aid.into(),
                    attr_key: base_attr_key,
                }),
                false => None,
            }
        }
    }
}
