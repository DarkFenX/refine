use crate::misc::{PValue, Value};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub(crate) struct Xyz {
    pub(crate) x: Value,
    pub(crate) y: Value,
    pub(crate) z: Value,
}
impl Xyz {
    pub(crate) fn get_vector_dot_product(self, rhs: Self) -> Value {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub(crate) fn get_vector_magnitude(self) -> PValue {
        PValue::from_value_unchecked((self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt())
    }
    pub(crate) fn get_vector_unit(self) -> Self {
        let magnitude = self.get_vector_magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
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
