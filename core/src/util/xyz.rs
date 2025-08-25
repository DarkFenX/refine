use ordered_float::Float;

use crate::def::AttrVal;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) struct Xyz {
    pub(crate) x: AttrVal,
    pub(crate) y: AttrVal,
    pub(crate) z: AttrVal,
}
impl Xyz {
    pub(crate) fn get_vector_dot_product(self, rhs: Self) -> AttrVal {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub(crate) fn get_vector_magnitude(self) -> AttrVal {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}
impl std::ops::Mul<AttrVal> for Xyz {
    type Output = Self;

    fn mul(self, rhs: AttrVal) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
