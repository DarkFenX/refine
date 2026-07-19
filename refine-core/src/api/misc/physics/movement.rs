use angle_sc::{Angle, Degrees, trig};

use crate::{
    misc::Xyz,
    num::{PValue, Value},
    ud::UPhysics,
};

#[derive(Copy, Clone)]
pub struct Movement {
    /// Movement direction of an item.
    pub direction: Direction,
    /// Portion of speed item moves with, relatively its max speed.
    pub speed: PValue,
}

#[derive(Copy, Clone)]
pub struct Direction {
    /// Degrees relatively X axis counter-clockwise.
    pub azimuth: Value,
    /// Degrees of elevation.
    pub elevation: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Movement {
    pub(in crate::api) fn from_u_physics(u_physics: &UPhysics) -> Self {
        Self {
            direction: Direction::from_xyz(u_physics.direction),
            speed: u_physics.speed,
        }
    }
}

impl Direction {
    pub(in crate::api) fn from_xyz(xyz: Xyz) -> Self {
        Self {
            azimuth: Value::from_f64(
                trig::arctan2d(
                    trig::UnitNegRange::clamp(xyz.y.into_f64()),
                    trig::UnitNegRange::clamp(xyz.x.into_f64()),
                )
                .0,
            ),
            elevation: Value::from_f64(
                trig::arctan2d(
                    trig::UnitNegRange::clamp(xyz.z.into_f64()),
                    trig::UnitNegRange::clamp((xyz.x.into_f64().powi(2) + xyz.y.into_f64().powi(2)).sqrt()),
                )
                .0,
            ),
        }
    }
    pub(in crate::api) fn into_xyz(self) -> Xyz {
        let azimuth = Angle::from(Degrees(self.azimuth.into_f64()));
        let elevation = Angle::from(Degrees(self.elevation.into_f64()));
        let az_sin = azimuth.sin().0;
        let az_cos = azimuth.cos().0;
        let el_sin = elevation.sin().0;
        let el_cos = elevation.cos().0;
        Xyz {
            x: Value::from_f64(az_cos * el_cos),
            y: Value::from_f64(az_sin * el_cos),
            z: Value::from_f64(el_sin),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::{
        de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
        ser::{Serialize, SerializeTuple, Serializer},
    };

    use super::*;

    impl Serialize for Movement {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tuple = serializer.serialize_tuple(3)?;
            tuple.serialize_element(&self.direction.azimuth)?;
            tuple.serialize_element(&self.direction.elevation)?;
            tuple.serialize_element(&self.speed)?;
            tuple.end()
        }
    }

    impl<'de> Deserialize<'de> for Movement {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorState;

            impl<'de> Visitor<'de> for VisitorState {
                type Value = Movement;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("sequence with 3 elements")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: SeqAccess<'de>,
                {
                    let azimuth = match SeqAccess::next_element(&mut seq)? {
                        Some(azimuth) => azimuth,
                        None => return Err(Error::invalid_length(0, &"sequence with 3 elements")),
                    };
                    let elevation = match SeqAccess::next_element(&mut seq)? {
                        Some(elevation) => elevation,
                        None => return Err(Error::invalid_length(1, &"sequence with 3 elements")),
                    };
                    let speed = match SeqAccess::next_element(&mut seq)? {
                        Some(speed) => speed,
                        None => return Err(Error::invalid_length(2, &"sequence with 3 elements")),
                    };
                    Ok(Movement {
                        direction: Direction { azimuth, elevation },
                        speed,
                    })
                }
            }

            deserializer.deserialize_seq(VisitorState)
        }
    }
}
