use crate::{
    ad::{AItemId, AMuta},
    util::RMap,
};

pub struct AMutas {
    pub(crate) data: RMap<AItemId, AMuta>,
}
impl AMutas {
    pub fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub fn insert(&mut self, val: AMuta) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AMuta> {
        self.data.values()
    }
}
impl FromIterator<AMuta> for AMutas {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AMuta>,
    {
        Self {
            data: iter.into_iter().map(|v| (v.id, v)).collect(),
        }
    }
}
