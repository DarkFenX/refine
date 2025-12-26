#![allow(private_bounds)]

use crate::{
    def::AttrVal,
    misc::{DmgKinds, Ecm, MiningAmount},
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(in crate::svc::aggr) trait InstanceMulAssign {
    fn instance_mul_assign(&mut self, mult: AttrVal);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementations for T/"primitives"
////////////////////////////////////////////////////////////////////////////////////////////////////
impl InstanceMulAssign for AttrVal {
    fn instance_mul_assign(&mut self, mult: AttrVal) {
        *self *= mult;
    }
}

impl InstanceMulAssign for DmgKinds<AttrVal> {
    fn instance_mul_assign(&mut self, mult: AttrVal) {
        self.em *= mult;
        self.thermal *= mult;
        self.kinetic *= mult;
        self.explosive *= mult;
    }
}

impl InstanceMulAssign for MiningAmount {
    fn instance_mul_assign(&mut self, mult: AttrVal) {
        self.yield_ *= mult;
        self.drain *= mult;
    }
}

impl InstanceMulAssign for Ecm {
    fn instance_mul_assign(&mut self, mult: AttrVal) {
        self.radar *= mult;
        self.magnetometric *= mult;
        self.gravimetric *= mult;
        self.ladar *= mult;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementation for output
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + InstanceMulAssign,
{
    pub(in crate::svc::aggr) fn instance_mul_assign(&mut self, limit: AttrVal) {
        match self {
            Self::Simple(inner) => inner.instance_mul_assign(limit),
            Self::Complex(inner) => inner.instance_mul_assign(limit),
        }
    }
}

impl<T> OutputSimple<T>
where
    T: Copy + InstanceMulAssign,
{
    fn instance_mul_assign(&mut self, limit: AttrVal) {
        self.amount.instance_mul_assign(limit);
    }
}

impl<T> OutputComplex<T>
where
    T: Copy + InstanceMulAssign,
{
    fn instance_mul_assign(&mut self, limit: AttrVal) {
        self.amount.instance_mul_assign(limit);
    }
}
