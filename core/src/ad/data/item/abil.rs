use crate::ad::AAbilId;

pub struct AItemAbils {
    data: Vec<AAbilId>,
}
impl AItemAbils {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AAbilId) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AAbilId> {
        self.data.iter()
    }
}
impl Default for AItemAbils {
    fn default() -> Self {
        Self::new()
    }
}
impl FromIterator<AAbilId> for AItemAbils {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AAbilId>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}
