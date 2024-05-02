#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
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
    ModLow,
    ModMid,
    Mutaplasmid,
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
            rc::ad::AItemKind::FighterSquad => Self::FighterSquad,
            rc::ad::AItemKind::Implant => Self::Implant,
            rc::ad::AItemKind::ModHigh => Self::ModHigh,
            rc::ad::AItemKind::ModLow => Self::ModLow,
            rc::ad::AItemKind::ModMid => Self::ModMid,
            rc::ad::AItemKind::Mutaplasmid => Self::Mutaplasmid,
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
            CItemKind::FighterSquad => rc::ad::AItemKind::FighterSquad,
            CItemKind::Implant => rc::ad::AItemKind::Implant,
            CItemKind::ModHigh => rc::ad::AItemKind::ModHigh,
            CItemKind::ModLow => rc::ad::AItemKind::ModLow,
            CItemKind::ModMid => rc::ad::AItemKind::ModMid,
            CItemKind::Mutaplasmid => rc::ad::AItemKind::Mutaplasmid,
            CItemKind::Rig => rc::ad::AItemKind::Rig,
            CItemKind::Ship => rc::ad::AItemKind::Ship,
            CItemKind::Skill => rc::ad::AItemKind::Skill,
            CItemKind::Stance => rc::ad::AItemKind::Stance,
            CItemKind::Subsystem => rc::ad::AItemKind::Subsystem,
        }
    }
}
