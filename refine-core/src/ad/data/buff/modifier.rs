use crate::ad::{AAttrId, ABuffAffecteeFilter};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct ABuffModifier {
    pub affectee_filter: ABuffAffecteeFilter,
    pub affectee_attr_id: AAttrId,
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
pub struct ABuffModifiers {
    data: Vec<ABuffModifier>,
}
impl ABuffModifiers {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: ABuffModifier) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &ABuffModifier> {
        self.data.iter()
    }
}
impl FromIterator<ABuffModifier> for ABuffModifiers {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = ABuffModifier>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}
