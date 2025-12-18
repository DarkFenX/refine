use ordered_float::Float;

use crate::{def::AttrVal, util::FLOAT_TOLERANCE};

pub(crate) struct OutputSimple<T>
where
    T: Copy + Clone,
{
    pub(crate) amount: T,
    pub(crate) delay: AttrVal,
}
impl<T> OutputSimple<T>
where
    T: Copy + Clone,
{
    pub(super) fn get_total(&self) -> T {
        self.amount
    }
    pub(super) fn get_amount(&self) -> T {
        self.amount
    }
    pub(super) fn get_delay(&self) -> AttrVal {
        self.delay
    }
    pub(super) fn iter_output(&self) -> impl Iterator<Item = (AttrVal, T)> {
        OutputSimpleIter::new(self)
    }
}
impl OutputSimple<AttrVal> {
    pub(super) fn has_impact(&self) -> bool {
        self.amount.abs() > FLOAT_TOLERANCE
    }
    pub(super) fn absolute_impact(&self) -> AttrVal {
        self.amount.abs()
    }
    pub(super) fn add_amount(&mut self, amount: AttrVal) {
        self.amount += amount;
    }
}
impl<T> std::ops::Neg for OutputSimple<T>
where
    T: Copy + Clone + std::ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.amount = -self.amount;
        self
    }
}

struct OutputSimpleIter<'a, T>
where
    T: Copy + Clone,
{
    output: &'a OutputSimple<T>,
    done: bool,
}
impl<'a, T> OutputSimpleIter<'a, T>
where
    T: Copy + Clone,
{
    fn new(output: &'a OutputSimple<T>) -> Self {
        Self { output, done: false }
    }
}
impl<T> Iterator for OutputSimpleIter<'_, T>
where
    T: Copy + Clone,
{
    type Item = (AttrVal, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.done {
            true => None,
            false => {
                self.done = true;
                Some((self.output.delay, self.output.amount))
            }
        }
    }
}
