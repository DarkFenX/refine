use super::{has_impact::HasImpact, instance_duration::InstanceDuration, limit_amount::InstanceLimit};
use crate::{
    misc::DmgKinds,
    num::PValue,
    util::{LibDefault, LibMax},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Aggregation-specific implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HasImpact for DmgKinds<PValue> {
    fn has_impact(&self) -> bool {
        let Self {
            em,
            thermal,
            kinetic,
            explosive,
        } = self;
        *em != PValue::ZERO || *thermal != PValue::ZERO || *kinetic != PValue::ZERO || *explosive != PValue::ZERO
    }
}
impl InstanceDuration for DmgKinds<PValue> {
    fn get_duration(&self) -> PValue {
        PValue::ZERO
    }
    fn limit_duration(&mut self, _limit: PValue) {}
}
impl InstanceLimit for DmgKinds<PValue> {
    // No-op, since there is no logic to limit damage depending on target attrs
    fn instance_limit(&mut self, _limit: PValue) {}
}
impl LibDefault for DmgKinds<PValue> {
    fn lib_default() -> Self {
        Self {
            em: PValue::ZERO,
            thermal: PValue::ZERO,
            kinetic: PValue::ZERO,
            explosive: PValue::ZERO,
        }
    }
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
        let Self {
            em,
            thermal,
            kinetic,
            explosive,
        } = self;
        *em += rhs.em;
        *thermal += rhs.thermal;
        *kinetic += rhs.kinetic;
        *explosive += rhs.explosive;
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
        let Self {
            em,
            thermal,
            kinetic,
            explosive,
        } = self;
        *em *= rhs;
        *thermal *= rhs;
        *kinetic *= rhs;
        *explosive *= rhs;
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
