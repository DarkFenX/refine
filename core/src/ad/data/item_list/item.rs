use crate::ad::AItemId;

pub struct AItemListItemIds {
    data: Vec<AItemId>,
}
impl AItemListItemIds {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AItemId) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemId> {
        self.data.iter()
    }
}
impl FromIterator<AItemId> for AItemListItemIds {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemId>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemListItemIds {
    pub(in crate::ad) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(in crate::ad) fn clear(&mut self) {
        self.data.clear()
    }
}
