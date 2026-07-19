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
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use super::*;

    const ROLL_PREFIX: &str = "r";
    const ABS_PREFIX: &str = "a";

    impl<'de> serde::Deserialize<'de> for AttrMutation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = AttrMutation;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("number or string with number with optional type prefix")
                }

                // TODO: experiment with visit methods and decide if this many is necessary, or just
                // TODO: f64 / f64+i32 is sufficient
                fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }

                fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v as f64)))
                }
                fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Self::Value::Absolute(Value::from_f64(v)))
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if let Some(roll_str) = v.strip_prefix(ROLL_PREFIX) {
                        let roll = UnitInterval::from_str(roll_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::Roll(roll));
                    }
                    if let Some(abs_str) = v.strip_prefix(ABS_PREFIX) {
                        let abs = Value::from_str(abs_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::Absolute(abs));
                    }
                    let abs_str = Value::from_str(v).map_err(|e| serde::de::Error::custom(e))?;
                    Ok(Self::Value::Absolute(abs_str))
                }
            }
            deserializer.deserialize_any(Visitor)
        }
    }
}

// TODO: add full serialization support and merge with module above
#[cfg(feature = "serde")]
mod custom_serde_x1 {
    use super::*;

    impl<'de> serde::Deserialize<'de> for AddMutation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = AddMutation;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("integer with mutator ID, or sequence with mutator ID and attribute map")
                }
            }
            deserializer.deserialize_any(Visitor)
        }
    }
}

// TODO: add full serialization support and merge with module above
#[cfg(feature = "serde")]
mod custom_serde_x2 {
    use super::*;

    impl<'de> serde::Deserialize<'de> for ChangeMutation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = ChangeMutation;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str(
                        "integer with mutator ID, attribute map, or sequence with mutator ID and attribute map",
                    )
                }
            }
            deserializer.deserialize_any(Visitor)
        }
    }
}
