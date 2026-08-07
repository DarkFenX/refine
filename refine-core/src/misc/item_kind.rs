/// Covers all item kinds supported by the library.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, derive_more::Display)]
#[display(rename_all = "snake_case")]
pub enum ItemKind {
    Autocharge,
    Booster,
    Character,
    Charge,
    Drone,
    Fighter,
    FwEffect,
    Implant,
    Module,
    ProjEffect,
    Rig,
    Service,
    Ship,
    Skill,
    Stance,
    Subsystem,
    SwEffect,
}

/// Covers only item kinds which can be auto-detected by the library.
///
/// Is used in item kind mismatch validation, and internally by try-fit-items functionality.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, derive_more::Display)]
#[display(rename_all = "snake_case")]
pub enum DetectedItemKind {
    Booster,
    Character,
    Charge,
    Drone,
    Fighter,
    Implant,
    ModuleHigh,
    ModuleMid,
    ModuleLow,
    Rig,
    Service,
    Ship,
    Skill,
    Stance,
    Subsystem,
}
