use std::collections::HashMap;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct Muta {
    id: rc::ReeInt,
    item_map: HashMap<rc::ReeInt, rc::ReeInt>,
    attr_mods: HashMap<rc::ReeInt, MutaAttrRange>,
}
impl From<&rc::ad::AMuta> for Muta {
    fn from(value: &rc::ad::AMuta) -> Self {
        Muta {
            id: value.id,
            item_map: value.item_map.clone(),
            attr_mods: value.attr_mods.iter().map(|(k, v)| (*k, v.into())).collect(),
        }
    }
}
impl Into<rc::ad::AMuta> for &Muta {
    fn into(self) -> rc::ad::AMuta {
        rc::ad::AMuta {
            id: self.id,
            item_map: self.item_map.clone(),
            attr_mods: self.attr_mods.iter().map(|(k, v)| (*k, v.into())).collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
struct MutaAttrRange {
    min_mult: rc::ReeFloat,
    max_mult: rc::ReeFloat,
}
impl From<&rc::ad::AMutaAttrRange> for MutaAttrRange {
    fn from(value: &rc::ad::AMutaAttrRange) -> Self {
        MutaAttrRange {
            min_mult: value.min_mult,
            max_mult: value.max_mult,
        }
    }
}
impl Into<rc::ad::AMutaAttrRange> for &MutaAttrRange {
    fn into(self) -> rc::ad::AMutaAttrRange {
        rc::ad::AMutaAttrRange {
            min_mult: self.min_mult,
            max_mult: self.max_mult,
        }
    }
}
