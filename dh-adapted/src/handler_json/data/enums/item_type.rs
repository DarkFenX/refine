#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CItemType {
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
impl From<&rc::ad::AItemType> for CItemType {
    fn from(item_type: &rc::ad::AItemType) -> Self {
        match item_type {
            rc::ad::AItemType::Booster => Self::Booster,
            rc::ad::AItemType::Character => Self::Character,
            rc::ad::AItemType::Charge => Self::Charge,
            rc::ad::AItemType::Drone => Self::Drone,
            rc::ad::AItemType::EffectBeacon => Self::EffectBeacon,
            rc::ad::AItemType::FighterSquad => Self::FighterSquad,
            rc::ad::AItemType::Implant => Self::Implant,
            rc::ad::AItemType::ModHigh => Self::ModHigh,
            rc::ad::AItemType::ModLow => Self::ModLow,
            rc::ad::AItemType::ModMid => Self::ModMid,
            rc::ad::AItemType::Mutaplasmid => Self::Mutaplasmid,
            rc::ad::AItemType::Rig => Self::Rig,
            rc::ad::AItemType::Ship => Self::Ship,
            rc::ad::AItemType::Skill => Self::Skill,
            rc::ad::AItemType::Stance => Self::Stance,
            rc::ad::AItemType::Subsystem => Self::Subsystem,
        }
    }
}
impl Into<rc::ad::AItemType> for &CItemType {
    fn into(self) -> rc::ad::AItemType {
        match self {
            CItemType::Booster => rc::ad::AItemType::Booster,
            CItemType::Character => rc::ad::AItemType::Character,
            CItemType::Charge => rc::ad::AItemType::Charge,
            CItemType::Drone => rc::ad::AItemType::Drone,
            CItemType::EffectBeacon => rc::ad::AItemType::EffectBeacon,
            CItemType::FighterSquad => rc::ad::AItemType::FighterSquad,
            CItemType::Implant => rc::ad::AItemType::Implant,
            CItemType::ModHigh => rc::ad::AItemType::ModHigh,
            CItemType::ModLow => rc::ad::AItemType::ModLow,
            CItemType::ModMid => rc::ad::AItemType::ModMid,
            CItemType::Mutaplasmid => rc::ad::AItemType::Mutaplasmid,
            CItemType::Rig => rc::ad::AItemType::Rig,
            CItemType::Ship => rc::ad::AItemType::Ship,
            CItemType::Skill => rc::ad::AItemType::Skill,
            CItemType::Stance => rc::ad::AItemType::Stance,
            CItemType::Subsystem => rc::ad::AItemType::Subsystem,
        }
    }
}
