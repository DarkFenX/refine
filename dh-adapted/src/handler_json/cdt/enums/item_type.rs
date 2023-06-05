#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum ItemType {
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
impl From<&rc::consts::ItemType> for ItemType {
    fn from(value: &rc::consts::ItemType) -> Self {
        match value {
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
impl Into<rc::consts::ItemType> for &ItemType {
    fn into(self) -> rc::consts::ItemType {
        match self {
            ItemType::Booster => rc::consts::ItemType::Booster,
            ItemType::Character => rc::consts::ItemType::Character,
            ItemType::Charge => rc::consts::ItemType::Charge,
            ItemType::Drone => rc::consts::ItemType::Drone,
            ItemType::EffectBeacon => rc::consts::ItemType::EffectBeacon,
            ItemType::FighterSquad => rc::consts::ItemType::FighterSquad,
            ItemType::Implant => rc::consts::ItemType::Implant,
            ItemType::ModHigh => rc::consts::ItemType::ModHigh,
            ItemType::ModLow => rc::consts::ItemType::ModLow,
            ItemType::ModMid => rc::consts::ItemType::ModMid,
            ItemType::Mutaplasmid => rc::consts::ItemType::Mutaplasmid,
            ItemType::Rig => rc::consts::ItemType::Rig,
            ItemType::Ship => rc::consts::ItemType::Ship,
            ItemType::Skill => rc::consts::ItemType::Skill,
            ItemType::Stance => rc::consts::ItemType::Stance,
            ItemType::Subsystem => rc::consts::ItemType::Subsystem,
        }
    }
}
