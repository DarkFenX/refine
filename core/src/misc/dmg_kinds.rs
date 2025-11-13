use crate::def::AttrVal;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DmgKinds<T> {
    pub em: T,
    pub thermal: T,
    pub kinetic: T,
    pub explosive: T,
}
impl<T> DmgKinds<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        DmgKindsIter::new(self)
    }
}
impl<T> DmgKinds<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}
impl<T> Default for DmgKinds<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            em: T::default(),
            thermal: T::default(),
            kinetic: T::default(),
            explosive: T::default(),
        }
    }
}
impl<T> std::ops::AddAssign<DmgKinds<T>> for DmgKinds<T>
where
    T: std::ops::AddAssign<T>,
{
    fn add_assign(&mut self, rhs: DmgKinds<T>) {
        self.em += rhs.em;
        self.thermal += rhs.thermal;
        self.kinetic += rhs.kinetic;
        self.explosive += rhs.explosive;
    }
}
impl<T> std::ops::Mul<AttrVal> for DmgKinds<T>
where
    T: std::ops::Mul<AttrVal, Output = T>,
{
    type Output = DmgKinds<T>;
    fn mul(self, rhs: AttrVal) -> Self::Output {
        Self {
            em: self.em * rhs,
            thermal: self.thermal * rhs,
            kinetic: self.kinetic * rhs,
            explosive: self.explosive * rhs,
        }
    }
}
impl<T> std::ops::Div<AttrVal> for DmgKinds<T>
where
    T: std::ops::Div<AttrVal, Output = T>,
{
    type Output = DmgKinds<T>;
    fn div(self, rhs: AttrVal) -> Self::Output {
        Self {
            em: self.em / rhs,
            thermal: self.thermal / rhs,
            kinetic: self.kinetic / rhs,
            explosive: self.explosive / rhs,
        }
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

pub struct DmgKindsIter<'a, T> {
    item: &'a DmgKinds<T>,
    index: usize,
}
impl<'a, T> DmgKindsIter<'a, T> {
    pub(super) fn new(item: &'a DmgKinds<T>) -> Self {
        Self { item, index: 0 }
    }
}
impl<'a, T> Iterator for DmgKindsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..4 => {
                let result = &self.item[self.index];
                self.index += 1;
                Some(result)
            }
            _ => None,
        }
    }
}
