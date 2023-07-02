use crate::handler_json::data::{CModOp, CModTgtFilter};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CEffectAttrMod {
    afor_attr_id: rc::EAttrId,
    op: CModOp,
    afee_filter: CModTgtFilter,
    afee_attr_id: rc::EAttrId,
}
impl From<&rc::ad::AEffectAttrMod> for CEffectAttrMod {
    fn from(a_modifier: &rc::ad::AEffectAttrMod) -> Self {
        CEffectAttrMod {
            afor_attr_id: a_modifier.afor_attr_id,
            op: (&a_modifier.op).into(),
            afee_filter: (&a_modifier.afee_filter).into(),
            afee_attr_id: a_modifier.afee_attr_id,
        }
    }
}
impl Into<rc::ad::AEffectAttrMod> for &CEffectAttrMod {
    fn into(self) -> rc::ad::AEffectAttrMod {
        rc::ad::AEffectAttrMod {
            afor_attr_id: self.afor_attr_id,
            op: (&self.op).into(),
            afee_filter: (&self.afee_filter).into(),
            afee_attr_id: self.afee_attr_id,
        }
    }
}
