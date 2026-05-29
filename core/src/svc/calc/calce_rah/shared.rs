use smallvec::SmallVec;

pub(super) type TickCount = usize;
pub(super) const TICK_LIMIT: TickCount = 500;
pub(super) const SIG_ROUND_DIGITS: u32 = 10;

////////////////////////////////////////////////////////////////////////////////////////////////////
// RAH-specific, but still general data container, which is optimized for storing 1 entry
////////////////////////////////////////////////////////////////////////////////////////////////////
// TODO: check if it's actually shared, move if necessary
#[derive(Clone, Eq, PartialEq, Hash)]
pub(super) struct ItemDataVec<T> {
    data: SmallVec<[T; 1]>,
}
impl<T> ItemDataVec<T> {
    pub(super) fn new() -> Self {
        Self { data: SmallVec::new() }
    }
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            data: SmallVec::with_capacity(capacity),
        }
    }
    pub(super) fn get(&self, index: usize) -> &T {
        // Logic in RAH sim guarantees that stored item count coincides to simulated item count
        self.data.get(index).unwrap()
    }
    pub(super) fn get_mut(&mut self, index: usize) -> &mut T {
        // Logic in RAH sim guarantees that stored item count coincides to simulated item count
        self.data.get_mut(index).unwrap()
    }
    pub(super) fn push(&mut self, value: T) {
        self.data.push(value)
    }
    pub(super) fn iter(&self) -> impl ExactSizeIterator<Item = &T> {
        self.data.iter()
    }
    pub(super) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut T> {
        self.data.iter_mut()
    }
    pub(super) fn len(&self) -> usize {
        self.data.len()
    }
    pub(super) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(super) fn clear(&mut self) {
        self.data.clear()
    }
}
