#![allow(private_bounds)]

use crate::{
    def::AttrVal,
    misc::MiningAmount,
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(super) trait InstanceLimit {
    fn instance_limit(&mut self, limit: AttrVal);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementations for T/"primitives"
////////////////////////////////////////////////////////////////////////////////////////////////////
impl InstanceLimit for AttrVal {
    fn instance_limit(&mut self, limit: AttrVal) {
        *self = AttrVal::min(*self, limit);
    }
}

impl InstanceLimit for MiningAmount {
    // No-op for mining
    fn instance_limit(&mut self, _limit: AttrVal) {}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementation for output
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + InstanceLimit,
{
    pub(super) fn instance_limit(&mut self, limit: AttrVal) {
        match self {
            Self::Simple(inner) => inner.instance_limit(limit),
            Self::Complex(inner) => inner.instance_limit(limit),
        }
    }
}

impl<T> OutputSimple<T>
where
    T: Copy + InstanceLimit,
{
    fn instance_limit(&mut self, limit: AttrVal) {
        self.amount.instance_limit(limit);
    }
}

impl<T> OutputComplex<T>
where
    T: Copy + InstanceLimit,
{
    fn instance_limit(&mut self, limit: AttrVal) {
        self.amount.instance_limit(limit);
    }
}
