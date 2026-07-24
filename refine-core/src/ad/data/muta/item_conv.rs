use crate::ad::AItemId;

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AMutaItemConv {
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
pub struct AMutaItemConvs {
    data: Vec<AMutaItemConv>,
}
impl AMutaItemConvs {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AMutaItemConv) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AMutaItemConv> {
        self.data.iter()
    }
}
impl FromIterator<AMutaItemConv> for AMutaItemConvs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AMutaItemConv>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}
