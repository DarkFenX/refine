/// Contains adapted item types.
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
pub enum AFighterKind {
    Support,
    Light,
    Heavy,
    StandupSupport,
    StandupLight,
    StandupHeavy,
}
