use crate::ad::{AAttrId, AMutaAttrRange};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AMutaAttr {
    pub attr_id: AAttrId,
    pub range: AMutaAttrRange,
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct AMutaAttrs {
    data: Vec<AMutaAttr>,
}
const impl Default for AMutaAttrs {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
impl AMutaAttrs {
    pub const fn new() -> Self {
        Self::default()
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
