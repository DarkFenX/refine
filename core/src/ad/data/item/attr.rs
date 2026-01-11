use crate::{
    ad::{AAttrId, AValue},
    util::CMap,
};

pub struct AItemAttr {
    pub id: AAttrId,
    pub value: AValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct AItemAttrs {
    data: CMap<AAttrId, AItemAttr>,
}
impl AItemAttrs {
    pub const fn new() -> Self {
        Self {
            data: CMap::const_new(),
        }
    }
    pub fn insert(&mut self, val: AItemAttr) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemAttr> {
        self.data.values()
    }
}
impl FromIterator<AItemAttr> for AItemAttrs {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemAttr>,
    {
        Self {
            data: CMap::const_from_iter(iter.into_iter().map(|v| (v.id, v))),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemAttrs {
    pub(in crate::ad) fn get(&self, id: &AAttrId) -> Option<&AItemAttr> {
        self.data.get(id)
    }
    pub(in crate::ad) fn contains_id(&self, id: &AAttrId) -> bool {
        self.data.contains_key(id)
    }
    pub(in crate::ad) fn entry(&mut self, id: AAttrId) -> std::collections::hash_map::Entry<'_, AAttrId, AItemAttr> {
        self.data.entry(id)
    }
}
