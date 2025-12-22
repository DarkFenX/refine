use crate::{def::AttrVal, util::InfCount};

pub(crate) struct CycleChargedInfo {
    pub(crate) repeat_count: InfCount,
    pub(crate) charged: Option<AttrVal>,
}

pub(crate) enum CycleChargedInfoIter {
    One(std::array::IntoIter<CycleChargedInfo, 1>),
    Two(std::array::IntoIter<CycleChargedInfo, 2>),
    Three(std::array::IntoIter<CycleChargedInfo, 3>),
}
impl Iterator for CycleChargedInfoIter {
    type Item = CycleChargedInfo;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::One(inner) => inner.next(),
            Self::Two(inner) => inner.next(),
            Self::Three(inner) => inner.next(),
        }
    }
}
impl ExactSizeIterator for CycleChargedInfoIter {
    fn len(&self) -> usize {
        match self {
            Self::One(inner) => inner.len(),
            Self::Two(inner) => inner.len(),
            Self::Three(inner) => inner.len(),
        }
    }
}
