use super::{output::Output, output_complex::OutputComplex, output_simple::OutputSimple};
use crate::num::{Count, PValue};

////////////////////////////////////////////////////////////////////////////////////////////////////
// High-level interface
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> Output<I> {
    pub(in crate::svc) fn into_instance_iter(self) -> OutputInstanceIter<I> {
        match self {
            Self::Simple(inner) => OutputInstanceIter::Simple(inner.into_instance_iter()),
            Self::Complex(inner) => OutputInstanceIter::Complex(inner.into_instance_iter()),
        }
    }
}

pub(in crate::svc) struct OutputInstanceIterItem<I> {
    pub(in crate::svc) time_passed: PValue,
    pub(in crate::svc) instance: I,
}

pub(in crate::svc) enum OutputInstanceIter<I> {
    Simple(OutputInstanceIterSimple<I>),
    Complex(OutputInstanceIterComplex<I>),
}
impl<I> Iterator for OutputInstanceIter<I>
where
    I: Copy,
{
    type Item = OutputInstanceIterItem<I>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Simple(inner) => inner.next(),
            Self::Complex(inner) => inner.next(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Simple
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> OutputSimple<I> {
    fn into_instance_iter(self) -> OutputInstanceIterSimple<I> {
        OutputInstanceIterSimple::new(self)
    }
}

pub(in crate::svc) struct OutputInstanceIterSimple<I> {
    output: OutputSimple<I>,
    done: bool,
}
impl<I> OutputInstanceIterSimple<I> {
    fn new(output: OutputSimple<I>) -> Self {
        Self { output, done: false }
    }
}
impl<I> Iterator for OutputInstanceIterSimple<I>
where
    I: Copy,
{
    type Item = OutputInstanceIterItem<I>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.done {
            true => None,
            false => {
                self.done = true;
                Some(OutputInstanceIterItem {
                    time_passed: self.output.delay,
                    instance: self.output.instance,
                })
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Complex
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> OutputComplex<I> {
    pub(super) fn into_instance_iter(self) -> OutputInstanceIterComplex<I> {
        OutputInstanceIterComplex::new(self)
    }
}

pub(in crate::svc) struct OutputInstanceIterComplex<I> {
    output: OutputComplex<I>,
    cycles_done: Count,
}
impl<I> OutputInstanceIterComplex<I> {
    fn new(output: OutputComplex<I>) -> Self {
        Self {
            output,
            cycles_done: Count::ZERO,
        }
    }
}
impl<I> Iterator for OutputInstanceIterComplex<I>
where
    I: Copy,
{
    type Item = OutputInstanceIterItem<I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles_done >= self.output.repeats {
            return None;
        }
        if self.cycles_done == Count::ZERO {
            self.cycles_done += Count::ONE;
            return Some(OutputInstanceIterItem {
                time_passed: self.output.delay,
                instance: self.output.instance,
            });
        }
        self.cycles_done += Count::ONE;
        Some(OutputInstanceIterItem {
            time_passed: self.output.interval,
            instance: self.output.instance,
        })
    }
}
