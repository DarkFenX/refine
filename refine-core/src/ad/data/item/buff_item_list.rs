use crate::ad::AItemListId;

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct AItemBuffItemLists {
    data: Vec<AItemListId>,
}
const impl Default for AItemBuffItemLists {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
impl AItemBuffItemLists {
    pub const fn new() -> Self {
        Self::default()
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
