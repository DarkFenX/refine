// Used to:
// - store validation-specific options in public validation options
// - store and access validation-specific options in internal validation options
// - variant names are used to deserialize options
// Because of deserialization part it affects public API despite being non-public; be careful when
// changing.
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc::vast::val) enum ValKind {
    // Generic
    NotLoadedItem,
    ItemKind,
    SkillReqs,
    // Implants/boosters
    ImplantSlotIndex,
    BoosterSlotIndex,
    // Shared between mod-alike items
    Cpu,
    Powergrid,
    ShipLimit,
    MaxGroupFitted,
    MaxGroupOnline,
    MaxGroupActive,
    MaxTypeFitted,
    ItemVsShipKind,
    // Modules
    HighSlotCount,
    MidSlotCount,
    LowSlotCount,
    TurretSlotCount,
    LauncherSlotCount,
    ModuleState,
    CapitalModule,
    OverloadSkill,
    UnusableCap,
    // Charges
    ChargeGroup,
    ChargeParentGroup,
    ChargeSize,
    ChargeVolume,
    // Rigs
    RigSlotCount,
    Calibration,
    RigSize,
    // Services
    ServiceSlotCount,
    // T3 subsystems/stances
    SubsystemSlotCount,
    SubsystemSlotIndex,
    ShipStance,
    // Drones
    DroneBayVolume,
    LaunchedDroneCount,
    DroneBandwidth,
    UnlaunchableDroneSlot,
    UnlaunchableDroneBandwidth,
    DroneGroup,
    // Fighters
    FighterBayVolume,
    LaunchedFighterCount,
    LaunchedLightFighterCount,
    LaunchedHeavyFighterCount,
    LaunchedSupportFighterCount,
    LaunchedStLightFighterCount,
    LaunchedStHeavyFighterCount,
    LaunchedStSupportFighterCount,
    UnlaunchableFighter,
    UnlaunchableLightFighter,
    UnlaunchableHeavyFighter,
    UnlaunchableSupportFighter,
    UnlaunchableStLightFighter,
    UnlaunchableStHeavyFighter,
    UnlaunchableStSupportFighter,
    FighterSquadSize,
    // Projection, destination side
    ActivationBlocked,
    EffectStopper,
    CloakingBlocked,
    // Projection, source side
    ProjecteeFilter,
    AssistImmunity,
    OffenseImmunity,
    ResistImmunity,
    // Sec zone
    SecZoneFitted,
    SecZoneOnline,
    SecZoneActive,
    SecZoneUnonlineable,
    SecZoneUnactivable,
    SecZoneEffect,
}
