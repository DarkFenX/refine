use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValItemKindFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    item_kinds: HashMap<rc::ItemId, HValItemKindItemInfo>,
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

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
struct HValItemKindItemInfo {
    kind: Option<HItemKind>,
    expected_kind: HItemKind,
}
impl From<&rc::val::ValItemKindItemInfo> for HValItemKindItemInfo {
    fn from(core_val_item_info: &rc::val::ValItemKindItemInfo) -> Self {
        Self {
            kind: core_val_item_info.kind.map(|v| (&v).into()),
            expected_kind: (&core_val_item_info.expected_kind).into(),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum HItemKind {
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
impl From<&rc::val::ItemKind> for HItemKind {
    fn from(a_item_kind: &rc::val::ItemKind) -> Self {
        match a_item_kind {
            rc::val::ItemKind::Booster => Self::Booster,
            rc::val::ItemKind::Character => Self::Character,
            rc::val::ItemKind::Charge => Self::Charge,
            rc::val::ItemKind::Drone => Self::Drone,
            rc::val::ItemKind::Fighter => Self::Fighter,
            rc::val::ItemKind::Implant => Self::Implant,
            rc::val::ItemKind::ModuleHigh => Self::ModuleHigh,
            rc::val::ItemKind::ModuleMid => Self::ModuleMid,
            rc::val::ItemKind::ModuleLow => Self::ModuleLow,
            rc::val::ItemKind::Rig => Self::Rig,
            rc::val::ItemKind::Service => Self::Service,
            rc::val::ItemKind::Ship => Self::Ship,
            rc::val::ItemKind::Skill => Self::Skill,
            rc::val::ItemKind::Stance => Self::Stance,
            rc::val::ItemKind::Subsystem => Self::Subsystem,
        }
    }
}
