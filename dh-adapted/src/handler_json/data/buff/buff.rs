use crate::handler_json::data::{CBuffAggrMode, CBuffModifier, COp};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CBuff {
    id: rc::EBuffId,
    aggr_mode: CBuffAggrMode,
    op: COp,
    mods: Vec<CBuffModifier>,
}
impl From<&rc::ad::ABuff> for CBuff {
    fn from(a_buff: &rc::ad::ABuff) -> Self {
        Self {
            id: a_buff.id,
            aggr_mode: (&a_buff.aggr_mode).into(),
            op: (&a_buff.op).into(),
            mods: a_buff.mods.iter().map(|v| v.into()).collect(),
        }
    }
}
impl From<&CBuff> for rc::ad::ABuff {
    fn from(c_buff: &CBuff) -> Self {
        Self {
            id: c_buff.id,
            aggr_mode: (&c_buff.aggr_mode).into(),
            op: (&c_buff.op).into(),
            mods: c_buff.mods.iter().map(|v| v.into()).collect(),
        }
    }
}
