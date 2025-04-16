use itertools::Itertools;

use crate::{
    ad,
    sol::{
        AttrId, AttrVal, OpInfo,
        api::{FullSideEffect, FullSideEffectMut},
    },
    src::Src,
};

pub struct SideEffectPartialStr {
    op: OpInfo,
    attr_id: AttrId,
}
impl SideEffectPartialStr {
    pub fn get_op(&self) -> OpInfo {
        self.op
    }
    pub fn get_attr_id(&self) -> AttrId {
        self.attr_id
    }
}

pub struct SideEffectStr {
    op: OpInfo,
    attr_id: AttrId,
    value: AttrVal,
}
impl SideEffectStr {
    pub fn get_op(&self) -> OpInfo {
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
        get_strength_partial(&self.sol.uad.src, &self.a_effect_id)
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Get side effect strength as an operator and attribute ID which contains strength value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength_partial(&self) -> Option<SideEffectPartialStr> {
        get_strength_partial(&self.sol.uad.src, &self.a_effect_id)
    }
    /// Get side effect strength as an operator and modification value.
    ///
    /// Returns something only if all the side effect modifiers use the same operator and attribute
    /// ID to apply modification.
    pub fn get_strength(&mut self) -> Option<SideEffectStr> {
        match self.get_strength_partial() {
            Some(partial) => match self.sol.internal_get_item_attr(self.key, &partial.attr_id) {
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

fn get_strength_partial(src: &Src, a_effect_id: &ad::AEffectId) -> Option<SideEffectPartialStr> {
    let mut se_strs = src
        .get_a_effect(a_effect_id)
        .unwrap()
        .mods
        .iter()
        .map(|a_modifier| (a_modifier.op, a_modifier.affector_attr_id))
        .collect_vec();
    match se_strs.len() {
        0 => None,
        1 => se_strs
            .into_iter()
            .map(|(a_op, a_attr_id)| SideEffectPartialStr {
                op: a_op.into(),
                attr_id: a_attr_id,
            })
            .next(),
        _ => {
            let (base_a_op, base_a_attr_id) = se_strs.pop().unwrap();
            match se_strs
                .into_iter()
                .all(|(a_op, a_attr_id)| a_op == base_a_op && a_attr_id == base_a_attr_id)
            {
                true => Some(SideEffectPartialStr {
                    op: base_a_op.into(),
                    attr_id: base_a_attr_id,
                }),
                false => None,
            }
        }
    }
}
