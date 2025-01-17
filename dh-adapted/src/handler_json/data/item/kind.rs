#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CItemKind {
    Booster,
    Character,
    Charge,
    Drone,
    EffectBeacon,
    FighterSquad(CFighterKind),
    Implant,
    ModHigh,
    ModLow,
    ModMid,
    Mutator,
    Rig,
    Ship,
    Skill,
    Stance,
    Subsystem,
}
impl From<&rc::ad::AItemKind> for CItemKind {
    fn from(item_kind: &rc::ad::AItemKind) -> Self {
        match item_kind {
            rc::ad::AItemKind::Booster => Self::Booster,
            rc::ad::AItemKind::Character => Self::Character,
            rc::ad::AItemKind::Charge => Self::Charge,
            rc::ad::AItemKind::Drone => Self::Drone,
            rc::ad::AItemKind::EffectBeacon => Self::EffectBeacon,
            rc::ad::AItemKind::FighterSquad(fighter_kind) => Self::FighterSquad(fighter_kind.into()),
            rc::ad::AItemKind::Implant => Self::Implant,
            rc::ad::AItemKind::ModHigh => Self::ModHigh,
            rc::ad::AItemKind::ModLow => Self::ModLow,
            rc::ad::AItemKind::ModMid => Self::ModMid,
            rc::ad::AItemKind::Mutator => Self::Mutator,
            rc::ad::AItemKind::Rig => Self::Rig,
            rc::ad::AItemKind::Ship => Self::Ship,
            rc::ad::AItemKind::Skill => Self::Skill,
            rc::ad::AItemKind::Stance => Self::Stance,
            rc::ad::AItemKind::Subsystem => Self::Subsystem,
        }
    }
}
impl Into<rc::ad::AItemKind> for &CItemKind {
    fn into(self) -> rc::ad::AItemKind {
        match self {
            CItemKind::Booster => rc::ad::AItemKind::Booster,
            CItemKind::Character => rc::ad::AItemKind::Character,
            CItemKind::Charge => rc::ad::AItemKind::Charge,
            CItemKind::Drone => rc::ad::AItemKind::Drone,
            CItemKind::EffectBeacon => rc::ad::AItemKind::EffectBeacon,
            CItemKind::FighterSquad(fighter_kind) => rc::ad::AItemKind::FighterSquad(fighter_kind.into()),
            CItemKind::Implant => rc::ad::AItemKind::Implant,
            CItemKind::ModHigh => rc::ad::AItemKind::ModHigh,
            CItemKind::ModLow => rc::ad::AItemKind::ModLow,
            CItemKind::ModMid => rc::ad::AItemKind::ModMid,
            CItemKind::Mutator => rc::ad::AItemKind::Mutator,
            CItemKind::Rig => rc::ad::AItemKind::Rig,
            CItemKind::Ship => rc::ad::AItemKind::Ship,
            CItemKind::Skill => rc::ad::AItemKind::Skill,
            CItemKind::Stance => rc::ad::AItemKind::Stance,
            CItemKind::Subsystem => rc::ad::AItemKind::Subsystem,
        }
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CFighterKind {
    Support,
    Light,
    Heavy,
    StandupSupport,
    StandupLight,
    StandupHeavy,
}
impl From<&rc::ad::AFighterKind> for CFighterKind {
    fn from(fighter_kind: &rc::ad::AFighterKind) -> Self {
        match fighter_kind {
            rc::ad::AFighterKind::Support => Self::Support,
            rc::ad::AFighterKind::Light => Self::Light,
            rc::ad::AFighterKind::Heavy => Self::Heavy,
            rc::ad::AFighterKind::StandupSupport => Self::StandupSupport,
            rc::ad::AFighterKind::StandupLight => Self::StandupLight,
            rc::ad::AFighterKind::StandupHeavy => Self::StandupHeavy,
        }
    }
}
impl Into<rc::ad::AFighterKind> for &CFighterKind {
    fn into(self) -> rc::ad::AFighterKind {
        match self {
            CFighterKind::Support => rc::ad::AFighterKind::Support,
            CFighterKind::Light => rc::ad::AFighterKind::Light,
            CFighterKind::Heavy => rc::ad::AFighterKind::Heavy,
            CFighterKind::StandupSupport => rc::ad::AFighterKind::StandupSupport,
            CFighterKind::StandupLight => rc::ad::AFighterKind::StandupLight,
            CFighterKind::StandupHeavy => rc::ad::AFighterKind::StandupHeavy,
        }
    }
}
