use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValItemVsShipKindFail {
    ship_kind: HShipKind,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    items: Vec<(rc::ItemId, HShipKind)>,
}
impl From<&rc::val::ValItemVsShipKindFail> for HValItemVsShipKindFail {
    fn from(core_val_fail: &rc::val::ValItemVsShipKindFail) -> Self {
        Self {
            ship_kind: (&core_val_fail.ship_kind).into(),
            items: core_val_fail
                .items
                .iter()
                .map(|(item_id, needed_kind)| (*item_id, needed_kind.into()))
                .collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum HShipKind {
    Ship,
    Structure,
    Unknown,
}
impl From<&rc::val::ValShipKind> for HShipKind {
    fn from(core_ship_kind: &rc::val::ValShipKind) -> Self {
        match core_ship_kind {
            rc::val::ValShipKind::Ship => Self::Ship,
            rc::val::ValShipKind::Structure => Self::Structure,
            rc::val::ValShipKind::Unknown => Self::Unknown,
        }
    }
}
