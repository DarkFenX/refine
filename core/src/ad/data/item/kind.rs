use crate::defs::SlotNumber;

/// Contains adapted item types.
#[derive(Copy, Clone)]
pub enum AItemKind {
    Booster(SlotNumber),
    Character,
    Charge,
    Drone,
    EffectBeacon,
    FighterSquad(AFighterKind),
    Implant(SlotNumber),
    ModHigh,
    ModLow,
    ModMid,
    Mutator,
    Rig,
    Ship,
    Skill,
    Stance,
    Subsystem(SlotNumber),
}

/// Contains adapted fighter squad types.
#[derive(Copy, Clone)]
pub enum AFighterKind {
    Support,
    Light,
    Heavy,
    StandupSupport,
    StandupLight,
    StandupHeavy,
}
