#[derive(serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub(in crate::handler_json) enum CItemKind {
    Booster,
    Character,
    Charge,
    Drone,
    EffectBeacon,
    FighterSquad,
    Implant,
    ModHigh,
    ModMid,
    ModLow,
    Mutator,
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
            rc::ad::AItemKind::EffectBeacon => Self::EffectBeacon,
            rc::ad::AItemKind::FighterSquad => Self::FighterSquad,
            rc::ad::AItemKind::Implant => Self::Implant,
            rc::ad::AItemKind::ModHigh => Self::ModHigh,
            rc::ad::AItemKind::ModMid => Self::ModMid,
            rc::ad::AItemKind::ModLow => Self::ModLow,
            rc::ad::AItemKind::Mutator => Self::Mutator,
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
            CItemKind::EffectBeacon => Self::EffectBeacon,
            CItemKind::FighterSquad => Self::FighterSquad,
            CItemKind::Implant => Self::Implant,
            CItemKind::ModHigh => Self::ModHigh,
            CItemKind::ModMid => Self::ModMid,
            CItemKind::ModLow => Self::ModLow,
            CItemKind::Mutator => Self::Mutator,
            CItemKind::Rig => Self::Rig,
            CItemKind::Ship => Self::Ship,
            CItemKind::Skill => Self::Skill,
            CItemKind::Stance => Self::Stance,
            CItemKind::Subsystem => Self::Subsystem,
        }
    }
}
