#![allow(private_bounds)]

use crate::{
    def::AttrVal,
    svc::output::{Output, OutputComplex, OutputSimple},
};

pub(in crate::svc::aggr) trait LimitAmount {
    fn limit_amount(&mut self, limit: AttrVal);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Exposure of appropriate methods in output
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> Output<T>
where
    T: Copy + LimitAmount,
{
    pub(in crate::svc::aggr) fn limit_amount(&mut self, limit: AttrVal) {
        match self {
            Self::Simple(inner) => inner.limit_amount(limit),
            Self::Complex(inner) => inner.limit_amount(limit),
        }
    }
}

impl<T> OutputSimple<T>
where
    T: Copy + LimitAmount,
{
    fn limit_amount(&mut self, limit: AttrVal) {
        self.amount.limit_amount(limit);
    }
}

impl<T> OutputComplex<T>
where
    T: Copy + LimitAmount,
{
    fn limit_amount(&mut self, limit: AttrVal) {
        self.amount.limit_amount(limit);
    }
}
