use angle_sc::{Angle, Degrees, trig};

use crate::{
    misc::{PValue, Value, Xyz},
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
    pub(in crate::api) fn from_u_physics(u_physics: UPhysics) -> Self {
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
