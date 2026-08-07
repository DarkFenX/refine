use crate::num::{PValue, UnitInterval};

/// Damage profile received by an item.
///
/// Absolute values of damage do not matter, only relative. It is used to affect RAH adaptation and
/// effective tank calculations.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpsProfile {
    pub em: PValue = PValue::ZERO,
    pub thermal: PValue = PValue::ZERO,
    pub kinetic: PValue = PValue::ZERO,
    pub explosive: PValue = PValue::ZERO,
    pub breacher: Option<BreacherProfile> = None,
}

/// Breacher part of damage profile received by an item.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BreacherProfile {
    pub absolute_max: PValue = PValue::ZERO,
    pub relative_max: UnitInterval = UnitInterval::ZERO,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DpsProfile {
    pub(crate) fn deals_breacher_dps(&self) -> bool {
        match self.breacher {
            Some(breacher) => breacher.relative_max > UnitInterval::ZERO && breacher.absolute_max > PValue::ZERO,
            None => false,
        }
    }
    pub(crate) fn get_sum_regular(&self) -> PValue {
        self.em + self.thermal + self.kinetic + self.explosive
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde_dps {
    use serde::{
        de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
        ser::{Serialize, SerializeSeq, Serializer},
    };

    use super::*;

    impl Serialize for DpsProfile {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_seq(None)?;
            seq.serialize_element(&self.em)?;
            seq.serialize_element(&self.thermal)?;
            seq.serialize_element(&self.kinetic)?;
            seq.serialize_element(&self.explosive)?;
            if let Some(breacher) = &self.breacher {
                seq.serialize_element(breacher)?;
            }
            seq.end()
        }
    }

    impl<'de> Deserialize<'de> for DpsProfile {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = DpsProfile;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with 4 or 5 elements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    Ok(Self::Value {
                        em: seq.next_element()?.ok_or(Error::invalid_length(0, &self))?,
                        thermal: seq.next_element()?.ok_or(Error::invalid_length(1, &self))?,
                        kinetic: seq.next_element()?.ok_or(Error::invalid_length(2, &self))?,
                        explosive: seq.next_element()?.ok_or(Error::invalid_length(3, &self))?,
                        breacher: seq.next_element()?,
                    })
                }
            }

            deserializer.deserialize_seq(VisitorImpl)
        }
    }
}

#[cfg(feature = "serde")]
mod custom_serde_breacher {
    use serde::{
        de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
        ser::{Serialize, SerializeTuple, Serializer},
    };

    use super::*;

    const FIELDS: usize = 2;

    impl Serialize for BreacherProfile {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = serializer.serialize_tuple(FIELDS)?;
            tuple.serialize_element(&self.absolute_max)?;
            tuple.serialize_element(&self.relative_max)?;
            tuple.end()
        }
    }

    impl<'de> Deserialize<'de> for BreacherProfile {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = BreacherProfile;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("tuple with 2 elements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    Ok(Self::Value {
                        absolute_max: seq.next_element()?.ok_or(Error::invalid_length(0, &self))?,
                        relative_max: seq.next_element()?.ok_or(Error::invalid_length(1, &self))?,
                    })
                }
            }

            deserializer.deserialize_tuple(FIELDS, VisitorImpl)
        }
    }
}
