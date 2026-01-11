use crate::ad::AAttrId;

pub struct AItemCapUseAttrs {
    data: Vec<AAttrId>,
}
impl AItemCapUseAttrs {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AAttrId) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AAttrId> {
        self.data.iter()
    }
}
impl FromIterator<AAttrId> for AItemCapUseAttrs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AAttrId>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemCapUseAttrs {
    pub(in crate::ad) fn contains(&self, val: &AAttrId) -> bool {
        self.data.contains(val)
    }
}
