use crate::util::State5;

#[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
pub(crate) struct DmgKinds<T> {
    pub(crate) em: T,
    pub(crate) thermal: T,
    pub(crate) kinetic: T,
    pub(crate) explosive: T,
}
impl<T> DmgKinds<T> {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        DmgKindsIter::new(self)
    }
    pub(crate) fn get_total(&self) -> T
    where
        T: Copy + std::ops::Add<T, Output = T>,
    {
        self.em + self.thermal + self.kinetic + self.explosive
    }
}
impl<T> std::ops::Index<usize> for DmgKinds<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.em,
            1 => &self.thermal,
            2 => &self.kinetic,
            3 => &self.explosive,
            n => panic!("invalid DmgKinds index: {n}"),
        }
    }
}
impl<T> std::ops::IndexMut<usize> for DmgKinds<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.em,
            1 => &mut self.thermal,
            2 => &mut self.kinetic,
            3 => &mut self.explosive,
            n => panic!("invalid DmgKinds index: {n}"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Iterator
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) struct DmgKindsIter<'a, T> {
    item: &'a DmgKinds<T>,
    state: State5,
}
impl<'a, T> DmgKindsIter<'a, T> {
    pub(in crate::misc) fn new(item: &'a DmgKinds<T>) -> Self {
        Self {
            item,
            state: State5::One,
        }
    }
}
impl<'a, T> Iterator for DmgKindsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State5::One => {
                self.state = State5::Two;
                Some(self.item.em)
            }
            State5::Two => {
                self.state = State5::Three;
                Some(self.item.thermal)
            }
            State5::Three => {
                self.state = State5::Four;
                Some(self.item.kinetic)
            }
            State5::Four => {
                self.state = State5::Five;
                Some(self.item.explosive)
            }
            State5::Five => None,
        }
    }
}
