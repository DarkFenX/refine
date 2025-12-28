use super::{aggregable::Aggregable, limit_amount::LimitAmount, maximum::Maximum};
use crate::{def::AttrVal, misc::DmgKinds};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Aggregable for DmgKinds<AttrVal> {}
impl LimitAmount for DmgKinds<AttrVal> {
    // No-op, since there is no logic to limit damage depending on target attrs
    fn limit_amount(&mut self, _limit: AttrVal) {}
}
impl Maximum for DmgKinds<AttrVal> {
    fn maximum(self, other: Self) -> Self {
        match self.get_total() >= other.get_total() {
            true => self,
            false => other,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Arithmetic operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> std::ops::AddAssign<DmgKinds<T>> for DmgKinds<T>
where
    T: std::ops::AddAssign<T>,
{
    fn add_assign(&mut self, rhs: DmgKinds<T>) {
        self.em += rhs.em;
        self.thermal += rhs.thermal;
        self.kinetic += rhs.kinetic;
        self.explosive += rhs.explosive;
    }
}
impl<T> std::ops::Mul<AttrVal> for DmgKinds<T>
where
    T: std::ops::Mul<AttrVal, Output = T>,
{
    type Output = DmgKinds<T>;
    fn mul(self, rhs: AttrVal) -> Self::Output {
        Self {
            em: self.em * rhs,
            thermal: self.thermal * rhs,
            kinetic: self.kinetic * rhs,
            explosive: self.explosive * rhs,
        }
    }
}
impl<T> std::ops::MulAssign<AttrVal> for DmgKinds<T>
where
    T: std::ops::MulAssign<AttrVal>,
{
    fn mul_assign(&mut self, rhs: AttrVal) {
        self.em *= rhs;
        self.thermal *= rhs;
        self.kinetic *= rhs;
        self.explosive *= rhs;
    }
}
impl<T> std::ops::Div<AttrVal> for DmgKinds<T>
where
    T: std::ops::Div<AttrVal, Output = T>,
{
    type Output = DmgKinds<T>;
    fn div(self, rhs: AttrVal) -> Self::Output {
        Self {
            em: self.em / rhs,
            thermal: self.thermal / rhs,
            kinetic: self.kinetic / rhs,
            explosive: self.explosive / rhs,
        }
    }
}
