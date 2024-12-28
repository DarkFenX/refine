#[derive(Copy, Clone)]
pub struct SolDmgTypes<T> {
    pub em: T,
    pub thermal: T,
    pub kinetic: T,
    pub explosive: T,
}
impl<T> SolDmgTypes<T> {
    pub(crate) fn new(em: T, thermal: T, kinetic: T, explosive: T) -> Self {
        Self {
            em,
            thermal,
            kinetic,
            explosive,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        SolDmgTypesIter::new(self)
    }
}
impl<T> std::ops::Index<usize> for SolDmgTypes<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.em,
            1 => &self.thermal,
            2 => &self.kinetic,
            3 => &self.explosive,
            n => panic!("invalid SolDmgTypes index: {}", n),
        }
    }
}
impl<T> std::ops::IndexMut<usize> for SolDmgTypes<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.em,
            1 => &mut self.thermal,
            2 => &mut self.kinetic,
            3 => &mut self.explosive,
            n => panic!("invalid SolDmgTypes index: {}", n),
        }
    }
}

pub struct SolDmgTypesIter<'a, T> {
    item: &'a SolDmgTypes<T>,
    index: usize,
}
impl<'a, T> SolDmgTypesIter<'a, T> {
    pub(super) fn new(item: &'a SolDmgTypes<T>) -> Self {
        Self { item, index: 0 }
    }
}
impl<'a, T> Iterator for SolDmgTypesIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.index {
                0..4 => {
                    let result = &self.item[self.index];
                    self.index += 1;
                    return Some(result)
                },
                _ => return None,
            };
        }
    }
}
