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

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum HShipKind {
    Ship,
    Structure,
    Unknown,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValItemVsShipKindFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValItemVsShipKindFail) -> Self {
        Self {
            ship_kind: HShipKind::from_core(core_val_fail.ship_kind),
            items: core_val_fail
                .items
                .into_iter()
                .map(|(item_id, needed_kind)| (item_id, HShipKind::from_core(needed_kind)))
                .collect(),
        }
    }
}

impl HShipKind {
    fn from_core(core_ship_kind: rc::val::ValShipKind) -> Self {
        match core_ship_kind {
            rc::val::ValShipKind::Ship => Self::Ship,
            rc::val::ValShipKind::Structure => Self::Structure,
            rc::val::ValShipKind::Unknown => Self::Unknown,
        }
    }
}
