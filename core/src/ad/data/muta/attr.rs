use crate::ad::{AAttrId, AMutaAttrRange};

pub struct AMutaAttr {
    pub attr_id: AAttrId,
    pub range: AMutaAttrRange,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct AMutaAttrs {
    data: Vec<AMutaAttr>,
}
impl AMutaAttrs {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AMutaAttr) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AMutaAttr> {
        self.data.iter()
    }
}
impl FromIterator<AMutaAttr> for AMutaAttrs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AMutaAttr>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}
