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
impl From<&rc::consts::ItemType> for CItemType {
    fn from(item_type: &rc::consts::ItemType) -> Self {
        match item_type {
            rc::consts::ItemType::Booster => Self::Booster,
            rc::consts::ItemType::Character => Self::Character,
            rc::consts::ItemType::Charge => Self::Charge,
            rc::consts::ItemType::Drone => Self::Drone,
            rc::consts::ItemType::EffectBeacon => Self::EffectBeacon,
            rc::consts::ItemType::FighterSquad => Self::FighterSquad,
            rc::consts::ItemType::Implant => Self::Implant,
            rc::consts::ItemType::ModHigh => Self::ModHigh,
            rc::consts::ItemType::ModLow => Self::ModLow,
            rc::consts::ItemType::ModMid => Self::ModMid,
            rc::consts::ItemType::Mutaplasmid => Self::Mutaplasmid,
            rc::consts::ItemType::Rig => Self::Rig,
            rc::consts::ItemType::Ship => Self::Ship,
            rc::consts::ItemType::Skill => Self::Skill,
            rc::consts::ItemType::Stance => Self::Stance,
            rc::consts::ItemType::Subsystem => Self::Subsystem,
        }
    }
}
impl Into<rc::consts::ItemType> for &CItemType {
    fn into(self) -> rc::consts::ItemType {
        match self {
            CItemType::Booster => rc::consts::ItemType::Booster,
            CItemType::Character => rc::consts::ItemType::Character,
            CItemType::Charge => rc::consts::ItemType::Charge,
            CItemType::Drone => rc::consts::ItemType::Drone,
            CItemType::EffectBeacon => rc::consts::ItemType::EffectBeacon,
            CItemType::FighterSquad => rc::consts::ItemType::FighterSquad,
            CItemType::Implant => rc::consts::ItemType::Implant,
            CItemType::ModHigh => rc::consts::ItemType::ModHigh,
            CItemType::ModLow => rc::consts::ItemType::ModLow,
            CItemType::ModMid => rc::consts::ItemType::ModMid,
            CItemType::Mutaplasmid => rc::consts::ItemType::Mutaplasmid,
            CItemType::Rig => rc::consts::ItemType::Rig,
            CItemType::Ship => rc::consts::ItemType::Ship,
            CItemType::Skill => rc::consts::ItemType::Skill,
            CItemType::Stance => rc::consts::ItemType::Stance,
            CItemType::Subsystem => rc::consts::ItemType::Subsystem,
        }
    }
}
