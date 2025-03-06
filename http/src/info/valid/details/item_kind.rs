use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValItemKindFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    data: HashMap<rc::SolItemId, HValItemKindItemInfo>,
}
impl HValItemKindFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolValItemKindFail>> for HValItemKindFail {
    fn from(core_val_fails: &Vec<rc::SolValItemKindFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.item_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValItemKindItemInfo {
    kind: Option<HItemKind>,
    expected_kind: HItemKind,
}
impl From<&rc::SolValItemKindFail> for HValItemKindItemInfo {
    fn from(core_val_fail: &rc::SolValItemKindFail) -> Self {
        Self {
            kind: core_val_fail.kind.map(|v| (&v).into()),
            expected_kind: (&core_val_fail.expected_kind).into(),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::info::valid) enum HItemKind {
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
            rc::ad::AItemKind::Ship => Self::Ship,
            rc::ad::AItemKind::Skill => Self::Skill,
            rc::ad::AItemKind::Stance => Self::Stance,
            rc::ad::AItemKind::Subsystem => Self::Subsystem,
        }
    }
}
