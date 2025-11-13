use ordered_float::Float;

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
    pub(super) fn get_amount(&self) -> T {
        self.amount
    }
    pub(super) fn get_delay(&self) -> AttrVal {
        self.delay
    }
    pub(super) fn iter_output(&self) -> impl Iterator<Item = (AttrVal, T)> {
        OutputComplexIter::new(self)
    }
}
impl<T> OutputComplex<T>
where
    T: Copy + Clone + std::ops::Mul<AttrVal, Output = T>,
{
    pub(super) fn get_total(&self) -> T {
        self.amount * OF(self.repeats as f64)
    }
}
impl OutputComplex<AttrVal> {
    pub(super) fn has_impact(&self) -> bool {
        self.amount != OF(0.0)
    }
    pub(super) fn absolute_impact(&self) -> AttrVal {
        self.amount.abs() * self.repeats as f64
    }
}
impl<T> std::ops::Neg for OutputComplex<T>
where
    T: Copy + Clone + std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.amount = -self.amount;
        self
    }
}

struct OutputComplexIter<'a, T>
where
    T: Copy + Clone,
{
    output: &'a OutputComplex<T>,
    cycles_done: Count,
}
impl<'a, T> OutputComplexIter<'a, T>
where
    T: Copy + Clone,
{
    fn new(output: &'a OutputComplex<T>) -> Self {
        Self { output, cycles_done: 0 }
    }
}
impl<T> Iterator for OutputComplexIter<'_, T>
where
    T: Copy + Clone,
{
    type Item = (AttrVal, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.cycles_done {
            0 => {
                self.cycles_done += 1;
                Some((self.output.delay, self.output.amount))
            }
            n if n <= self.output.repeats => {
                self.cycles_done += 1;
                Some((self.output.interval, self.output.amount))
            }
            _ => None,
        }
    }
}
