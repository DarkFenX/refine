#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SolDmgKinds<T> {
    pub em: T,
    pub thermal: T,
    pub kinetic: T,
    pub explosive: T,
}
impl<T> SolDmgKinds<T> {
    pub(crate) fn new(em: T, thermal: T, kinetic: T, explosive: T) -> Self {
        Self {
            em,
            thermal,
            kinetic,
            explosive,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        SolDmgKindsIter::new(self)
    }
}
impl<T> std::ops::Index<usize> for SolDmgKinds<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.em,
            1 => &self.thermal,
            2 => &self.kinetic,
            3 => &self.explosive,
            n => panic!("invalid SolDmgKinds index: {}", n),
        }
    }
}
impl<T> std::ops::IndexMut<usize> for SolDmgKinds<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.em,
            1 => &mut self.thermal,
            2 => &mut self.kinetic,
            3 => &mut self.explosive,
            n => panic!("invalid SolDmgKinds index: {}", n),
        }
    }
}

pub struct SolDmgKindsIter<'a, T> {
    item: &'a SolDmgKinds<T>,
    index: usize,
}
impl<'a, T> SolDmgKindsIter<'a, T> {
    pub(super) fn new(item: &'a SolDmgKinds<T>) -> Self {
        Self { item, index: 0 }
    }
}
impl<'a, T> Iterator for SolDmgKindsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.index {
                0..4 => {
                    let result = &self.item[self.index];
                    self.index += 1;
                    return Some(result);
                }
                _ => return None,
            };
        }
    }
}
