use crate::{svc::cycle::Cycle, util::InfCount};

impl<T> Cycle<T>
where
    T: Copy,
{
    pub(in crate::svc) fn iter_parts(&self) -> CyclePartIter<T> {
        match self {
            Self::Lim(inner) => inner.iter_parts(),
            Self::Inf(inner) => inner.iter_parts(),
            Self::LimInf(inner) => inner.iter_parts(),
            Self::LimSinInf(inner) => inner.iter_parts(),
            Self::LoopLimSin(inner) => inner.iter_parts(),
        }
    }
}

pub(crate) struct CyclePart<T> {
    pub(crate) data: T,
    pub(crate) repeat_count: InfCount,
}

pub(crate) enum CyclePartIter<T> {
    One(std::array::IntoIter<CyclePart<T>, 1>),
    Two(std::array::IntoIter<CyclePart<T>, 2>),
    Three(std::array::IntoIter<CyclePart<T>, 3>),
}
impl<T> Iterator for CyclePartIter<T> {
    type Item = CyclePart<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::One(inner) => inner.next(),
            Self::Two(inner) => inner.next(),
            Self::Three(inner) => inner.next(),
        }
    }
}
impl<T> ExactSizeIterator for CyclePartIter<T> {
    fn len(&self) -> usize {
        match self {
            Self::One(inner) => inner.len(),
            Self::Two(inner) => inner.len(),
            Self::Three(inner) => inner.len(),
        }
    }
}
