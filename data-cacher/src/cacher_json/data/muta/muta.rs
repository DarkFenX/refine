use std::collections::HashMap;

use crate::cacher_json::data::{CAttrId, CItemId, CMutaAttrRange};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CMuta {
    id: CItemId,
    item_map: HashMap<CItemId, CItemId>,
    attr_mods: HashMap<CAttrId, CMutaAttrRange>,
}
impl From<&rc::ad::AMuta> for CMuta {
    fn from(a_muta: &rc::ad::AMuta) -> Self {
        Self {
            id: a_muta.id,
            item_map: (&a_muta.item_map).into(),
            attr_mods: a_muta.attr_mods.iter().map(|(k, v)| (k.into(), v.into())).collect(),
        }
    }
}
impl From<&CMuta> for rc::ad::AMuta {
    fn from(c_muta: &CMuta) -> Self {
        Self {
            id: c_muta.id,
            item_map: (&c_muta.item_map).into(),
            attr_mods: c_muta.attr_mods.iter().map(|(k, v)| (k.into(), v.into())).collect(),
        }
    }
}
