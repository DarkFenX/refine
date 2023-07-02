use std::collections::HashMap;

use crate::handler_json::data::CMutaAttrRange;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CMuta {
    id: rc::EMutaId,
    item_map: HashMap<rc::EItemId, rc::EItemId>,
    attr_mods: HashMap<rc::EAttrId, CMutaAttrRange>,
}
impl From<&rc::ad::AMuta> for CMuta {
    fn from(a_muta: &rc::ad::AMuta) -> Self {
        CMuta {
            id: a_muta.id,
            item_map: a_muta.item_map.clone(),
            attr_mods: a_muta.attr_mods.iter().map(|(k, v)| (*k, v.into())).collect(),
        }
    }
}
impl Into<rc::ad::AMuta> for &CMuta {
    fn into(self) -> rc::ad::AMuta {
        rc::ad::AMuta {
            id: self.id,
            item_map: self.item_map.clone(),
            attr_mods: self.attr_mods.iter().map(|(k, v)| (*k, v.into())).collect(),
        }
    }
}
