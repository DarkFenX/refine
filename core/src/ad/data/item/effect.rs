use crate::{
    ad::{AEffectId, AItemEffectData},
    util::CMap,
};

pub struct AItemEffect {
    pub id: AEffectId,
    pub data: AItemEffectData = AItemEffectData::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct AItemEffects {
    data: CMap<AEffectId, AItemEffect>,
}
impl AItemEffects {
    pub const fn new() -> Self {
        Self {
            data: CMap::const_new(),
        }
    }
    pub fn insert(&mut self, val: AItemEffect) {
        self.data.insert(val.id, val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AItemEffect> {
        self.data.values()
    }
}
impl FromIterator<AItemEffect> for AItemEffects {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AItemEffect>,
    {
        Self {
            data: CMap::const_from_iter(iter.into_iter().map(|v| (v.id, v))),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemEffects {
    pub(crate) fn contains_id(&self, id: &AEffectId) -> bool {
        self.data.contains_key(id)
    }
    pub(in crate::ad) fn keys(&self) -> impl ExactSizeIterator<Item = &AEffectId> {
        self.data.keys()
    }
    pub(in crate::ad) fn get_mut(&mut self, id: &AEffectId) -> Option<&mut AItemEffect> {
        self.data.get_mut(id)
    }
    pub(in crate::ad) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut AItemEffect> {
        self.data.values_mut()
    }
}
