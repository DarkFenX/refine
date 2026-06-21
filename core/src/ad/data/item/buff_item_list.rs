use crate::ad::AItemListId;

#[derive(Default)]
pub struct AItemBuffItemLists {
    data: Vec<AItemListId>,
}
impl AItemBuffItemLists {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AItemListId) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemListId> {
        self.data.iter()
    }
}
impl FromIterator<AItemListId> for AItemBuffItemLists {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemListId>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemBuffItemLists {
    pub(in crate::ad) fn len(&self) -> usize {
        self.data.len()
    }
}
