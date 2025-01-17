/// Contains adapted item types.
#[derive(Copy, Clone)]
pub enum AItemKind {
    Booster,
    Character,
    Charge,
    Drone,
    EffectBeacon,
    FighterSquad(AFighterKind),
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
