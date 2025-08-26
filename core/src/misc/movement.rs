use angle_sc::{Angle, Degrees, trig};
use ordered_float::Float;

use crate::{
    def::{AttrVal, OF},
    ud::UPhysics,
    util::Xyz,
};

#[derive(Copy, Clone)]
pub struct Movement {
    /// Movement direction of an item.
    pub direction: Direction,
    /// Portion of speed item moves with, relatively its max speed.
    pub speed: AttrVal,
}
impl From<&UPhysics> for Movement {
    fn from(u_physics: &UPhysics) -> Self {
        Self {
            direction: u_physics.direction.into(),
            speed: u_physics.speed,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Direction {
    /// Degrees relatively X axis counter-clockwise.
    pub azimuth: AttrVal,
    /// Degrees of elevation.
    pub elevation: AttrVal,
}
impl From<Xyz> for Direction {
    fn from(direction: Xyz) -> Self {
        Self {
            azimuth: OF(trig::arctan2d(
                trig::UnitNegRange::clamp(direction.y.into_inner()),
                trig::UnitNegRange::clamp(direction.x.into_inner()),
            )
            .0),
            elevation: OF(trig::arctan2d(
                trig::UnitNegRange::clamp(direction.z.into_inner()),
                trig::UnitNegRange::clamp((direction.x.powi(2) + direction.y.powi(2)).sqrt().into_inner()),
            )
            .0),
        }
    }
}
impl From<Direction> for Xyz {
    fn from(direction: Direction) -> Self {
        let azimuth = Angle::from(Degrees(direction.azimuth.into_inner()));
        let elevation = Angle::from(Degrees(direction.elevation.into_inner()));
        let az_sin = OF(azimuth.sin().0);
        let az_cos = OF(azimuth.cos().0);
        let el_sin = OF(elevation.sin().0);
        let el_cos = OF(elevation.cos().0);
        Self {
            x: az_cos * el_cos,
            y: az_sin * el_cos,
            z: el_sin,
        }
    }
}
