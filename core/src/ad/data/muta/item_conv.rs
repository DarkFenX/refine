use crate::ad::AItemId;

pub struct AMutaItemConv {
    pub base_item_id: AItemId,
    pub mutated_item_id: AItemId,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
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
