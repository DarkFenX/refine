use crate::handler_json::data::CModTgtFilter;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CBuffAttrMod {
    afee_filter: CModTgtFilter,
    afee_attr_id: rc::EAttrId,
}
impl From<&rc::ad::ABuffAttrMod> for CBuffAttrMod {
    fn from(a_modifier: &rc::ad::ABuffAttrMod) -> Self {
        CBuffAttrMod {
            afee_filter: (&a_modifier.afee_filter).into(),
            afee_attr_id: a_modifier.afee_attr_id,
        }
    }
}
impl Into<rc::ad::ABuffAttrMod> for &CBuffAttrMod {
    fn into(self) -> rc::ad::ABuffAttrMod {
        rc::ad::ABuffAttrMod {
            afee_filter: (&self.afee_filter).into(),
            afee_attr_id: self.afee_attr_id,
        }
    }
}
