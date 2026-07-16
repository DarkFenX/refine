use crate::{AttrId, ItemTypeId, UnitInterval, Value};

#[derive(Copy, Clone)]
pub enum AttrMutation {
    Roll(UnitInterval),
    Absolute(Value),
}

pub struct AddMutation {
    pub mutator_id: ItemTypeId,
    pub attrs: Vec<(AttrId, AttrMutation)> = Vec::new(),
}
impl AddMutation {
    pub fn new(mutator_id: ItemTypeId) -> Self {
        Self { mutator_id, .. }
    }
    pub fn with_attrs(mut self, attrs: impl Iterator<Item = (AttrId, AttrMutation)>) -> Self {
        self.attrs.clear();
        self.attrs.extend(attrs);
        self
    }
}

#[derive(Default)]
pub struct ChangeMutation {
    pub mutator_id: Option<ItemTypeId> = None,
    pub attrs: Vec<(AttrId, Option<AttrMutation>)> = Vec::new(),
}
impl ChangeMutation {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_mutator_id(mut self, mutator_id: ItemTypeId) -> Self {
        self.mutator_id = Some(mutator_id);
        self
    }
    pub fn with_attrs(mut self, attrs: impl Iterator<Item = (AttrId, Option<AttrMutation>)>) -> Self {
        self.attrs.clear();
        self.attrs.extend(attrs);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddMutation {
    pub(in crate::cmd) fn apply_attrs(&self, core_mutation: &mut rc::MutationMut) {
        for (attr_id, value) in self.attrs.iter() {
            match value {
                AttrMutation::Absolute(value) => apply_absolute(core_mutation, *attr_id, *value),
                AttrMutation::Roll(roll) => apply_roll(core_mutation, *attr_id, *roll),
            }
        }
    }
}

impl ChangeMutation {
    pub(in crate::cmd) fn apply_attrs(&self, core_mutation: &mut rc::MutationMut) {
        for (attr_id, value) in self.attrs.iter() {
            match value {
                Some(AttrMutation::Absolute(value)) => apply_absolute(core_mutation, *attr_id, *value),
                Some(AttrMutation::Roll(roll)) => apply_roll(core_mutation, *attr_id, *roll),
                None => {
                    if let Ok(core_raw_mattr) = core_mutation.get_raw_mattr_mut(*attr_id) {
                        core_raw_mattr.remove();
                    }
                }
            }
        }
    }
}

fn apply_absolute(core_mutation: &mut rc::MutationMut, attr_id: AttrId, value: Value) {
    // Absolute values can be applied only to effective mutations, via full mutated attributes
    if let rc::MutationMut::Effective(core_effective_mutation) = core_mutation
        && let Ok(mut core_full_mattr) = core_effective_mutation.get_full_mattr_mut(attr_id)
    {
        core_full_mattr.set_value(Some(value))
    }
}

fn apply_roll(core_mutation: &mut rc::MutationMut, attr_id: AttrId, roll: UnitInterval) {
    // Try to get raw attr, if it's not available - add it
    match core_mutation.get_raw_mattr_mut(attr_id) {
        Ok(mut core_raw_mattr) => {
            core_raw_mattr.set_roll(roll);
        }
        Err(rc::err::GetRawMAttrError::MutationNotFound(_)) => {
            core_mutation.mutate_raw(attr_id, roll).unwrap();
        }
    };
}
