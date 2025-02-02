#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::handler_json) enum CItemKind {
    Booster,
    Character,
    Charge,
    Drone,
    EffectBeacon,
    FighterSquad,
    Implant,
    Module(CModRack, CShipKind),
    Mutator,
    Rig(CShipKind),
    Ship(CShipKind),
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
            rc::ad::AItemKind::Module(mod_rack, ship_kind) => Self::Module(mod_rack.into(), ship_kind.into()),
            rc::ad::AItemKind::Mutator => Self::Mutator,
            rc::ad::AItemKind::Rig(ship_kind) => Self::Rig(ship_kind.into()),
            rc::ad::AItemKind::Ship(ship_kind) => Self::Ship(ship_kind.into()),
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
            CItemKind::Module(mod_rack, ship_kind) => rc::ad::AItemKind::Module(mod_rack.into(), ship_kind.into()),
            CItemKind::Mutator => rc::ad::AItemKind::Mutator,
            CItemKind::Rig(ship_kind) => rc::ad::AItemKind::Rig(ship_kind.into()),
            CItemKind::Ship(ship_kind) => rc::ad::AItemKind::Ship(ship_kind.into()),
            CItemKind::Skill => rc::ad::AItemKind::Skill,
            CItemKind::Stance => rc::ad::AItemKind::Stance,
            CItemKind::Subsystem => rc::ad::AItemKind::Subsystem,
        }
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CShipKind {
    Ship,
    CapitalShip,
    Structure,
}
impl From<&rc::ad::AShipKind> for CShipKind {
    fn from(ship_kind: &rc::ad::AShipKind) -> Self {
        match ship_kind {
            rc::ad::AShipKind::Ship => Self::Ship,
            rc::ad::AShipKind::CapitalShip => Self::CapitalShip,
            rc::ad::AShipKind::Structure => Self::Structure,
        }
    }
}
impl Into<rc::ad::AShipKind> for &CShipKind {
    fn into(self) -> rc::ad::AShipKind {
        match self {
            CShipKind::Ship => rc::ad::AShipKind::Ship,
            CShipKind::CapitalShip => rc::ad::AShipKind::CapitalShip,
            CShipKind::Structure => rc::ad::AShipKind::Structure,
        }
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub(in crate::handler_json) enum CModRack {
    High,
    Mid,
    Low,
}
impl From<&rc::ad::AModRack> for CModRack {
    fn from(mod_rack: &rc::ad::AModRack) -> Self {
        match mod_rack {
            rc::ad::AModRack::High => Self::High,
            rc::ad::AModRack::Mid => Self::Mid,
            rc::ad::AModRack::Low => Self::Low,
        }
    }
}
impl Into<rc::ad::AModRack> for &CModRack {
    fn into(self) -> rc::ad::AModRack {
        match self {
            CModRack::High => rc::ad::AModRack::High,
            CModRack::Mid => rc::ad::AModRack::Mid,
            CModRack::Low => rc::ad::AModRack::Low,
        }
    }
}
