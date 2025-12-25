#![allow(private_bounds)]

use crate::{
    def::AttrVal,
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(super) trait InstanceMul {
    fn instance_mul(self, mult: AttrVal) -> Self;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementations for T/"primitives"
////////////////////////////////////////////////////////////////////////////////////////////////////
impl InstanceMul for AttrVal {
    fn instance_mul(self, mult: AttrVal) -> Self {
        self * mult
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementation for output
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + InstanceMul,
{
    pub(super) fn instance_sum(&self) -> T {
        match self {
            Self::Simple(inner) => inner.instance_sum(),
            Self::Complex(inner) => inner.instance_sum(),
        }
    }
}

impl<T> OutputSimple<T>
where
    T: Copy,
{
    fn instance_sum(&self) -> T {
        self.amount
    }
}

impl<T> OutputComplex<T>
where
    T: Copy + InstanceMul,
{
    fn instance_sum(&self) -> T {
        self.amount.instance_mul(AttrVal::from(self.repeats))
    }
}
