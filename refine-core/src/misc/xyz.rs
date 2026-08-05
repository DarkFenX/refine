use crate::num::{PValue, Value};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub(crate) struct Xyz {
    pub(crate) x: Value,
    pub(crate) y: Value,
    pub(crate) z: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Math
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Xyz {
    pub(crate) fn get_vector_cross_product(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub(crate) fn get_vector_magnitude(self) -> PValue {
        PValue::from_value_unchecked((self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt())
    }
}
impl std::ops::Add<Xyz> for Xyz {
    type Output = Self;

    fn add(self, rhs: Xyz) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Sub<Xyz> for Xyz {
    type Output = Self;

    fn sub(self, rhs: Xyz) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Mul<Value> for Xyz {
    type Output = Self;

    fn mul(self, rhs: Value) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Mul<PValue> for Xyz {
    type Output = Self;

    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
