use crate::{AttrId, ItemTypeId, UnitInterval, Value};

#[derive(Clone)]
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

#[derive(Clone, Default)]
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

#[derive(Copy, Clone)]
pub enum AttrMutation {
    Roll(UnitInterval),
    Absolute(Value),
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization - adding mutation
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde_add {
    use serde::de::{Deserialize, Deserializer};

    use super::*;

    impl<'de> Deserialize<'de> for AddMutation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match MutationAddFormats::deserialize(deserializer)? {
                MutationAddFormats::Short(mutator_id) => Self { mutator_id, .. },
                MutationAddFormats::Full(mutator_id, attrs) => Self { mutator_id, attrs },
            })
        }
    }

    #[serde_with::serde_as]
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum MutationAddFormats {
        Short(ItemTypeId),
        Full(
            ItemTypeId,
            #[serde_as(as = "serde_with::Map<_, _>")] Vec<(AttrId, AttrMutation)>,
        ),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization - changing mutation
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde_change {
    use serde::de::{Deserialize, Deserializer};

    use super::*;

    impl<'de> Deserialize<'de> for ChangeMutation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match MutationChangeFormats::deserialize(deserializer)? {
                MutationChangeFormats::MutatorOnly(mutator_id) => Self {
                    mutator_id: Some(mutator_id),
                    ..
                },
                MutationChangeFormats::AttrsOnly(attrs) => Self { attrs, .. },
                MutationChangeFormats::Full(mutator_id, attrs) => Self {
                    mutator_id: Some(mutator_id),
                    attrs,
                },
            })
        }
    }

    #[serde_with::serde_as]
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum MutationChangeFormats {
        MutatorOnly(ItemTypeId),
        AttrsOnly(#[serde_as(as = "serde_with::Map<_, _>")] Vec<(AttrId, Option<AttrMutation>)>),
        Full(
            ItemTypeId,
            #[serde_as(as = "serde_with::Map<_, _>")] Vec<(AttrId, Option<AttrMutation>)>,
        ),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization - attribute definition
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde_attr {
    use std::str::FromStr;

    use serde::de::{Deserialize, Deserializer, Error, Visitor};

    use super::*;

    const ROLL_PREFIX: &str = "r";
    const ABS_PREFIX: &str = "a";

    impl<'de> Deserialize<'de> for AttrMutation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AttrMutation;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("number or string with number with optional type prefix")
                }

                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v)))
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    if let Some(roll_str) = v.strip_prefix(ROLL_PREFIX) {
                        let roll = UnitInterval::from_str(roll_str).map_err(|e| Error::custom(e))?;
                        return Ok(Self::Value::Roll(roll));
                    }
                    if let Some(abs_str) = v.strip_prefix(ABS_PREFIX) {
                        let abs = Value::from_str(abs_str).map_err(|e| Error::custom(e))?;
                        return Ok(Self::Value::Absolute(abs));
                    }
                    let abs_str = Value::from_str(v).map_err(|e| Error::custom(e))?;
                    Ok(Self::Value::Absolute(abs_str))
                }
            }
            deserializer.deserialize_any(VisitorImpl)
        }
    }
}
