use crate::def::{AttrVal, Count, OF};

pub(crate) struct OutputComplex<T>
where
    T: Copy + Clone,
{
    pub(crate) amount: T,
    pub(crate) delay: AttrVal,
    pub(crate) repeats: Count,
    pub(crate) interval: AttrVal,
}
impl<T> OutputComplex<T>
where
    T: Copy + Clone,
{
    pub(in crate::svc) fn get_max(&self) -> T {
        self.amount
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + Clone + std::ops::Mul<AttrVal, Output = T>,
{
    pub(in crate::svc) fn get_total(&self) -> T {
        self.amount * OF(self.repeats as f64)
    }
}
