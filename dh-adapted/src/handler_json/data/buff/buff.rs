use crate::handler_json::data::{CBuffAttrMod, CModAggrMode, CModOp};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CBuff {
    id: rc::EBuffId,
    aggr_mode: CModAggrMode,
    op: CModOp,
    mods: Vec<CBuffAttrMod>,
}
impl From<&rc::ad::ABuff> for CBuff {
    fn from(a_buff: &rc::ad::ABuff) -> Self {
        CBuff {
            id: a_buff.id,
            aggr_mode: (&a_buff.aggr_mode).into(),
            op: (&a_buff.op).into(),
            mods: a_buff.mods.iter().map(|v| v.into()).collect(),
        }
    }
}
impl Into<rc::ad::ABuff> for &CBuff {
    fn into(self) -> rc::ad::ABuff {
        rc::ad::ABuff {
            id: self.id,
            aggr_mode: (&self.aggr_mode).into(),
            op: (&self.op).into(),
            mods: self.mods.iter().map(|v| v.into()).collect(),
        }
    }
}
