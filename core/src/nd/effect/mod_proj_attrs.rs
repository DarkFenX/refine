use crate::ad::{AAttrId, AEffect};

pub(crate) enum NModProjAttrsGetter {
    Simple,
    Full,
    AoeDd,
    AoeBurst,
}
impl NModProjAttrsGetter {
    pub(crate) fn get(&self, a_effect: &AEffect) -> [Option<AAttrId>; 2] {
        match self {
            Self::Simple => [a_effect.range_attr_id, None],
            Self::Full => [a_effect.range_attr_id, a_effect.falloff_attr_id],
            Self::AoeDd => [Some(AAttrId::MAX_RANGE), None],
            Self::AoeBurst => [Some(AAttrId::MAX_RANGE), Some(AAttrId::DOOMSDAY_AOE_RANGE)],
        }
    }
}
