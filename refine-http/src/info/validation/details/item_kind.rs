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

#[serde_as]
#[derive(Serialize_tuple)]
struct HValItemKindItemInfo {
    kind: Option<HValItemKind>,
    expected_kind: HValItemKind,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValItemKindFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValItemKindFail) -> Self {
        Self {
            item_kinds: core_val_fail
                .item_kinds
                .into_iter()
                .map(|(item_id, core_item_info)| (item_id, HValItemKindItemInfo::from_core(core_item_info)))
                .collect(),
        }
    }
}

impl HValItemKindItemInfo {
    fn from_core(core_val_item_info: rc::val::ValItemKindItemInfo) -> Self {
        Self {
            kind: core_val_item_info.kind.map(HValItemKind::from_core),
            expected_kind: HValItemKind::from_core(core_val_item_info.expected_kind),
        }
    }
}

impl HValItemKind {
    fn from_core(core_item_kind: rc::val::ValItemKind) -> Self {
        match core_item_kind {
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
