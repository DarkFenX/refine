#![allow(private_bounds)]

use crate::{
    def::AttrVal,
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(super) trait InstanceMulAssign {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementation for output
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + InstanceMulAssign,
{
    pub(super) fn instance_mul_assign(&mut self, limit: AttrVal) {
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
