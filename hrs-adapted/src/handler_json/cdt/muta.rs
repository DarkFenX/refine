use std::collections::HashMap;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct Muta {
    id: rc::ReeInt,
    item_map: HashMap<rc::ReeInt, rc::ReeInt>,
    attr_mods: HashMap<rc::ReeInt, MutaAttrRange>,
}
impl From<&rc::adt::Muta> for Muta {
    fn from(value: &rc::adt::Muta) -> Self {
        Muta {
            id: value.id,
            item_map: value.item_map.clone(),
            attr_mods: value.attr_mods.iter().map(|(k, v)| (*k, v.into())).collect(),
        }
    }
}
impl Into<rc::adt::Muta> for &Muta {
    fn into(self) -> rc::adt::Muta {
        rc::adt::Muta {
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
impl From<&rc::adt::MutaAttrRange> for MutaAttrRange {
    fn from(value: &rc::adt::MutaAttrRange) -> Self {
        MutaAttrRange {
            min_mult: value.min_mult,
            max_mult: value.max_mult,
        }
    }
}
impl Into<rc::adt::MutaAttrRange> for &MutaAttrRange {
    fn into(self) -> rc::adt::MutaAttrRange {
        rc::adt::MutaAttrRange {
            min_mult: self.min_mult,
            max_mult: self.max_mult,
        }
    }
}
