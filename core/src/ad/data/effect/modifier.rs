use crate::ad::{AAttrId, AEffectAffecteeFilter, AOp};

pub struct AEffectModifier {
    pub affector_attr_id: AAttrId,
    pub op: AOp,
    pub affectee_filter: AEffectAffecteeFilter,
    pub affectee_attr_id: AAttrId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Container
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct AEffectModifiers {
    data: Vec<AEffectModifier>,
}
impl AEffectModifiers {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn insert(&mut self, val: AEffectModifier) {
        self.data.push(val);
    }
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &AEffectModifier> {
        self.data.iter()
    }
}
impl FromIterator<AEffectModifier> for AEffectModifiers {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = AEffectModifier>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AEffectModifiers {
    pub(crate) fn extend<I: IntoIterator<Item = AEffectModifier>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(crate) fn clear(&mut self) {
        self.data.clear()
    }
}
