use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValItemKindFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    item_kinds: Vec<(rc::ItemId, HValItemKindItemInfo)>,
}
impl From<&rc::val::ValItemKindFail> for HValItemKindFail {
    fn from(core_val_fail: &rc::val::ValItemKindFail) -> Self {
        Self {
            item_kinds: core_val_fail
                .item_kinds
                .iter()
                .map(|(item_id, core_item_info)| (*item_id, core_item_info.into()))
                .collect(),
        }
    }
}

#[serde_as]
#[derive(Serialize_tuple)]
struct HValItemKindItemInfo {
    kind: Option<HValItemKind>,
    expected_kind: HValItemKind,
}
impl From<&rc::val::ValItemKindItemInfo> for HValItemKindItemInfo {
    fn from(core_val_item_info: &rc::val::ValItemKindItemInfo) -> Self {
        Self {
            kind: core_val_item_info.kind.map(|v| (&v).into()),
            expected_kind: (&core_val_item_info.expected_kind).into(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum HValItemKind {
    Booster,
    Character,
    Charge,
    Drone,
    Fighter,
    Implant,
    ModuleHigh,
    ModuleMid,
    ModuleLow,
    Rig,
    Service,
    Ship,
    Skill,
    Stance,
    Subsystem,
}
impl From<&rc::val::ValItemKind> for HValItemKind {
    fn from(a_item_kind: &rc::val::ValItemKind) -> Self {
        match a_item_kind {
            rc::val::ValItemKind::Booster => Self::Booster,
            rc::val::ValItemKind::Character => Self::Character,
            rc::val::ValItemKind::Charge => Self::Charge,
            rc::val::ValItemKind::Drone => Self::Drone,
            rc::val::ValItemKind::Fighter => Self::Fighter,
            rc::val::ValItemKind::Implant => Self::Implant,
            rc::val::ValItemKind::ModuleHigh => Self::ModuleHigh,
            rc::val::ValItemKind::ModuleMid => Self::ModuleMid,
            rc::val::ValItemKind::ModuleLow => Self::ModuleLow,
            rc::val::ValItemKind::Rig => Self::Rig,
            rc::val::ValItemKind::Service => Self::Service,
            rc::val::ValItemKind::Ship => Self::Ship,
            rc::val::ValItemKind::Skill => Self::Skill,
            rc::val::ValItemKind::Stance => Self::Stance,
            rc::val::ValItemKind::Subsystem => Self::Subsystem,
        }
    }
}
