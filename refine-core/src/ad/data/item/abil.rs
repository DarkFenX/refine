use crate::ad::AAbilId;

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct AItemAbils {
    data: Vec<AAbilId>,
}
const impl Default for AItemAbils {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
impl AItemAbils {
    pub const fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, val: AAbilId) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AAbilId> {
        self.data.iter()
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
