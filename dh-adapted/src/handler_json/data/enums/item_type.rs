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
impl From<&rc::ItemType> for CItemType {
    fn from(item_type: &rc::ItemType) -> Self {
        match item_type {
            rc::ItemType::Booster => Self::Booster,
            rc::ItemType::Character => Self::Character,
            rc::ItemType::Charge => Self::Charge,
            rc::ItemType::Drone => Self::Drone,
            rc::ItemType::EffectBeacon => Self::EffectBeacon,
            rc::ItemType::FighterSquad => Self::FighterSquad,
            rc::ItemType::Implant => Self::Implant,
            rc::ItemType::ModHigh => Self::ModHigh,
            rc::ItemType::ModLow => Self::ModLow,
            rc::ItemType::ModMid => Self::ModMid,
            rc::ItemType::Mutaplasmid => Self::Mutaplasmid,
            rc::ItemType::Rig => Self::Rig,
            rc::ItemType::Ship => Self::Ship,
            rc::ItemType::Skill => Self::Skill,
            rc::ItemType::Stance => Self::Stance,
            rc::ItemType::Subsystem => Self::Subsystem,
        }
    }
}
impl Into<rc::ItemType> for &CItemType {
    fn into(self) -> rc::ItemType {
        match self {
            CItemType::Booster => rc::ItemType::Booster,
            CItemType::Character => rc::ItemType::Character,
            CItemType::Charge => rc::ItemType::Charge,
            CItemType::Drone => rc::ItemType::Drone,
            CItemType::EffectBeacon => rc::ItemType::EffectBeacon,
            CItemType::FighterSquad => rc::ItemType::FighterSquad,
            CItemType::Implant => rc::ItemType::Implant,
            CItemType::ModHigh => rc::ItemType::ModHigh,
            CItemType::ModLow => rc::ItemType::ModLow,
            CItemType::ModMid => rc::ItemType::ModMid,
            CItemType::Mutaplasmid => rc::ItemType::Mutaplasmid,
            CItemType::Rig => rc::ItemType::Rig,
            CItemType::Ship => rc::ItemType::Ship,
            CItemType::Skill => rc::ItemType::Skill,
            CItemType::Stance => rc::ItemType::Stance,
            CItemType::Subsystem => rc::ItemType::Subsystem,
        }
    }
}
