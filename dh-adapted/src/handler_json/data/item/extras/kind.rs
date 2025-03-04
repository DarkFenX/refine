#[derive(serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub(in crate::handler_json) enum CItemKind {
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
impl From<&rc::ad::AItemKind> for CItemKind {
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
impl From<&CItemKind> for rc::ad::AItemKind {
    fn from(c_item_kind: &CItemKind) -> Self {
        match c_item_kind {
            CItemKind::Booster => Self::Booster,
            CItemKind::Character => Self::Character,
            CItemKind::Charge => Self::Charge,
            CItemKind::Drone => Self::Drone,
            CItemKind::Fighter => Self::Fighter,
            CItemKind::Implant => Self::Implant,
            CItemKind::ModuleHigh => Self::ModuleHigh,
            CItemKind::ModuleMid => Self::ModuleMid,
            CItemKind::ModuleLow => Self::ModuleLow,
            CItemKind::Rig => Self::Rig,
            CItemKind::Ship => Self::Ship,
            CItemKind::Skill => Self::Skill,
            CItemKind::Stance => Self::Stance,
            CItemKind::Subsystem => Self::Subsystem,
        }
    }
}
