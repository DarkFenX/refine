use crate::num::{Count, PValue};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct OutputComplex<I> {
    pub(crate) instance: I,
    pub(crate) delay: PValue,
    // Total count of instances
    pub(crate) repeats: Count,
    pub(crate) interval: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// General operations
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> OutputComplex<I> {
    pub(super) fn get_instance(&self) -> I
    where
        I: Copy,
    {
        self.instance
    }
    pub(super) fn get_instance_count(&self) -> Count {
        self.repeats
    }
    pub(super) fn get_immediate_instance(&self) -> Option<I>
    where
        I: Copy,
    {
        match self.delay {
            PValue::ZERO => Some(self.instance),
            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Math
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> OutputComplex<I>
where
    I: std::ops::MulAssign<PValue>,
{
    pub(super) fn instance_mul_assign(&mut self, rhs: PValue) {
        self.instance.mul_assign(rhs);
    }
}
