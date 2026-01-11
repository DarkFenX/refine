use super::attr_range::CMutaAttrRange;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json::data) struct CMuta {
    id: i32,
    item_map: Vec<(i32, i32)>,
    attr_mods: Vec<CMutaAttr>,
}
impl CMuta {
    pub(in crate::cacher_json::data) fn from_adapted(a_muta: &rc::ad::AMuta) -> Self {
        Self {
            id: a_muta.id.into_i32(),
            item_map: a_muta
                .item_map
                .iter()
                .map(|v| (v.base_item_id.into_i32(), v.mutated_item_id.into_i32()))
                .collect(),
            attr_mods: a_muta
                .attr_mods
                .iter()
                .map(|v| CMutaAttr {
                    id: v.attr_id,
                    range: CMutaAttrRange::from_adapted(&v.range),
                })
                .collect(),
        }
    }
    pub(in crate::cacher_json::data) fn into_adapted(self) -> rc::ad::AMuta {
        rc::ad::AMuta {
            id: rc::ad::AItemId::from_i32(self.id),
            item_map: self
                .item_map
                .into_iter()
                .map(|(k, v)| rc::ad::AMutaItemConv {
                    base_item_id: rc::ad::AItemId::from_i32(k),
                    mutated_item_id: rc::ad::AItemId::from_i32(v),
                })
                .collect(),
            attr_mods: self
                .attr_mods
                .into_iter()
                .map(|v| rc::ad::AMutaAttr {
                    attr_id: v.id,
                    range: v.range.into_adapted(),
                })
                .collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct CMutaAttr {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ad::AAttrId,
    range: CMutaAttrRange,
}
