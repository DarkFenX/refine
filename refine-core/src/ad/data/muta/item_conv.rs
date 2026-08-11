use crate::ad::AItemId;

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AMutaItem {
    pub base_item_id: AItemId,
    pub mutated_item_id: AItemId,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Default)]
pub struct AMutaItems {
    data: Vec<AMutaItem>,
}
impl AMutaItems {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AMutaItem) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AMutaItem> {
        self.data.iter()
    }
}
impl FromIterator<AMutaItem> for AMutaItems {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AMutaItem>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}
