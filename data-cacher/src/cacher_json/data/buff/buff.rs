use super::{aggr_mode::CBuffAggrMode, modifier::CBuffModifier};
use crate::cacher_json::data::{AdaptedConv, COp};

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json::data) struct CBuff {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ad::ABuffId,
    aggr_mode: CBuffAggrMode,
    op: COp,
    mods: Vec<CBuffModifier>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CBuff {
    type AEntity = rc::ad::ABuff;

    fn from_adapted(a_buff: &Self::AEntity) -> Self {
        Self {
            id: a_buff.id,
            aggr_mode: CBuffAggrMode::from_adapted(&a_buff.aggr_mode),
            op: COp::from_adapted(&a_buff.op),
            mods: a_buff.mods.iter().map(CBuffModifier::from_adapted).collect(),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            id: self.id,
            aggr_mode: self.aggr_mode.into_adapted(),
            op: self.op.into_adapted(),
            mods: self.mods.into_iter().map(|v| v.into_adapted()).collect(),
        }
    }
}
