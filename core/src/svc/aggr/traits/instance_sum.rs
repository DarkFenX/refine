#![allow(private_bounds)]

use crate::{
    def::AttrVal,
    misc::{DmgKinds, Ecm, MiningAmount},
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(in crate::svc::aggr) trait InstanceMul {
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

impl InstanceMul for DmgKinds<AttrVal> {
    fn instance_mul(self, mult: AttrVal) -> Self {
        DmgKinds {
            em: self.em * mult,
            thermal: self.thermal * mult,
            kinetic: self.kinetic * mult,
            explosive: self.explosive * mult,
        }
    }
}

impl InstanceMul for MiningAmount {
    fn instance_mul(self, mult: AttrVal) -> Self {
        MiningAmount {
            yield_: self.yield_ * mult,
            drain: self.drain * mult,
        }
    }
}

// TODO: consider if ECM has to be here at all
impl InstanceMul for Ecm {
    fn instance_mul(self, mult: AttrVal) -> Self {
        Ecm {
            radar: self.radar * mult,
            magnetometric: self.magnetometric * mult,
            gravimetric: self.gravimetric * mult,
            ladar: self.ladar * mult,
            duration: self.duration,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Implementation for output
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc::aggr) fn instance_sum(&self) -> T {
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
    T: Copy + std::ops::Mul<AttrVal, Output = T>,
{
    fn instance_sum(&self) -> T {
        self.amount * AttrVal::from(self.repeats)
    }
}
