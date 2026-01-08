use super::limit_amount::LimitAmount;
use crate::{
    misc::{DmgKinds, PValue, Value},
    util::LibMax,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl LimitAmount for DmgKinds<PValue> {
    // No-op, since there is no logic to limit damage depending on target attrs
    fn limit_amount(&mut self, _limit: Value) {}
}
impl LibMax for DmgKinds<PValue> {
    fn lib_max(self, rhs: Self) -> Self {
        match self.get_total() >= rhs.get_total() {
            true => self,
            false => rhs,
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
impl<T> std::ops::Mul<PValue> for DmgKinds<T>
where
    T: std::ops::Mul<PValue, Output = T>,
{
    type Output = DmgKinds<T>;
    fn mul(self, rhs: PValue) -> Self::Output {
        Self {
            em: self.em * rhs,
            thermal: self.thermal * rhs,
            kinetic: self.kinetic * rhs,
            explosive: self.explosive * rhs,
        }
    }
}
impl<T> std::ops::MulAssign<PValue> for DmgKinds<T>
where
    T: std::ops::MulAssign<PValue>,
{
    fn mul_assign(&mut self, rhs: PValue) {
        self.em *= rhs;
        self.thermal *= rhs;
        self.kinetic *= rhs;
        self.explosive *= rhs;
    }
}
impl<T> std::ops::Div<PValue> for DmgKinds<T>
where
    T: std::ops::Div<PValue, Output = T>,
{
    type Output = DmgKinds<T>;
    fn div(self, rhs: PValue) -> Self::Output {
        Self {
            em: self.em / rhs,
            thermal: self.thermal / rhs,
            kinetic: self.kinetic / rhs,
            explosive: self.explosive / rhs,
        }
    }
}
