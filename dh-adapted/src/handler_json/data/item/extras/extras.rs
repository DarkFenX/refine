use crate::handler_json::data::{CItemKind, CItemShipLimit};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CItemExtras {
    pub(in crate::handler_json) kind: Option<CItemKind>,
    pub(in crate::handler_json) volume: Option<rc::AttrVal>,
    pub(in crate::handler_json) ship_limit: Option<CItemShipLimit>,
}
impl From<&rc::ad::AItemExtras> for CItemExtras {
    fn from(a_item_extras: &rc::ad::AItemExtras) -> Self {
        CItemExtras {
            kind: a_item_extras.kind.as_ref().map(|v| v.into()),
            volume: a_item_extras.volume,
            ship_limit: a_item_extras.ship_limit.as_ref().map(|v| v.into()),
        }
    }
}
impl Into<rc::ad::AItemExtras> for &CItemExtras {
    fn into(self) -> rc::ad::AItemExtras {
        rc::ad::AItemExtras {
            kind: self.kind.as_ref().map(|v| v.into()),
            volume: self.volume,
            ship_limit: self.ship_limit.as_ref().map(|v| v.into()),
        }
    }
}
