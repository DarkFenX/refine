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
impl From<&rc::ad::AItemKind> for HItemKind {
    fn from(a_item_kind: &rc::ad::AItemKind) -> Self {
        match a_item_kind {
            rc::ad::AItemKind::Booster => Self::Booster,
            rc::ad::AItemKind::Character => Self::Character,
            rc::ad::AItemKind::Charge => Self::Charge,
            rc::ad::AItemKind::Drone => Self::Drone,
            rc::ad::AItemKind::Fighter => Self::Fighter,
            rc::ad::AItemKind::Implant => Self::Implant,
            rc::ad::AItemKind::ModuleHigh => Self::ModuleHigh,
            rc::ad::AItemKind::ModuleMid => Self::ModuleMid,
            rc::ad::AItemKind::ModuleLow => Self::ModuleLow,
            rc::ad::AItemKind::Rig => Self::Rig,
            rc::ad::AItemKind::Service => Self::Service,
            rc::ad::AItemKind::Ship => Self::Ship,
            rc::ad::AItemKind::Skill => Self::Skill,
            rc::ad::AItemKind::Stance => Self::Stance,
            rc::ad::AItemKind::Subsystem => Self::Subsystem,
        }
    }
}
