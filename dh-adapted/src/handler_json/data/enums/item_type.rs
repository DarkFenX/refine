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
impl From<&rc::ad::ItemType> for CItemType {
    fn from(item_type: &rc::ad::ItemType) -> Self {
        match item_type {
            rc::ad::ItemType::Booster => Self::Booster,
            rc::ad::ItemType::Character => Self::Character,
            rc::ad::ItemType::Charge => Self::Charge,
            rc::ad::ItemType::Drone => Self::Drone,
            rc::ad::ItemType::EffectBeacon => Self::EffectBeacon,
            rc::ad::ItemType::FighterSquad => Self::FighterSquad,
            rc::ad::ItemType::Implant => Self::Implant,
            rc::ad::ItemType::ModHigh => Self::ModHigh,
            rc::ad::ItemType::ModLow => Self::ModLow,
            rc::ad::ItemType::ModMid => Self::ModMid,
            rc::ad::ItemType::Mutaplasmid => Self::Mutaplasmid,
            rc::ad::ItemType::Rig => Self::Rig,
            rc::ad::ItemType::Ship => Self::Ship,
            rc::ad::ItemType::Skill => Self::Skill,
            rc::ad::ItemType::Stance => Self::Stance,
            rc::ad::ItemType::Subsystem => Self::Subsystem,
        }
    }
}
impl Into<rc::ad::ItemType> for &CItemType {
    fn into(self) -> rc::ad::ItemType {
        match self {
            CItemType::Booster => rc::ad::ItemType::Booster,
            CItemType::Character => rc::ad::ItemType::Character,
            CItemType::Charge => rc::ad::ItemType::Charge,
            CItemType::Drone => rc::ad::ItemType::Drone,
            CItemType::EffectBeacon => rc::ad::ItemType::EffectBeacon,
            CItemType::FighterSquad => rc::ad::ItemType::FighterSquad,
            CItemType::Implant => rc::ad::ItemType::Implant,
            CItemType::ModHigh => rc::ad::ItemType::ModHigh,
            CItemType::ModLow => rc::ad::ItemType::ModLow,
            CItemType::ModMid => rc::ad::ItemType::ModMid,
            CItemType::Mutaplasmid => rc::ad::ItemType::Mutaplasmid,
            CItemType::Rig => rc::ad::ItemType::Rig,
            CItemType::Ship => rc::ad::ItemType::Ship,
            CItemType::Skill => rc::ad::ItemType::Skill,
            CItemType::Stance => rc::ad::ItemType::Stance,
            CItemType::Subsystem => rc::ad::ItemType::Subsystem,
        }
    }
}
