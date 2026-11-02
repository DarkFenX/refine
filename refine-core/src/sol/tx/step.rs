use crate::{
    CountNz, EffectMode, FitSecStatus, MinionState, ModuleState, MoveMode, NpcProp, OptionalReload, RearmMinion,
    RemoveMode, SecZone, ServiceState, SkillLevel, Spool,
    ad::{AEffectId, AItemId},
    ud::{UFitId, UFleetId, UItemId},
};

/// Data needed to revert a single change done to a solar system.
#[derive(Clone)]
pub(crate) enum UndoStep {
    // Solar system
    SecZone(SecZone),
    DefaultNpcProp(NpcProp),
    DefaultOptionalReloads(OptionalReload),
    DefaultRearmMinions(RearmMinion),
    DefaultSpool(Spool),
    DefaultIncomingDps,
    // Fleet
    FleetAdded(UFleetId),
    FleetRemoved(UFleetId),
    // Fit
    FitAdded(UFitId),
    FitRemoved(UFitId),
    FitFleet(UFitId, Option<UFleetId>),
    FitRahIncomingDps(UFitId),
    FitSecStatus(UFitId, FitSecStatus),
    // Item - general
    ItemEffectMode(UItemId, AEffectId, EffectMode),
    ProjAdded(UItemId, UItemId),   // Projector and projectee
    ProjRemoved(UItemId, UItemId), // Projector and projectee
    // Item - autocharge
    AutochargeState(UItemId, bool),
    // Item - booster
    BoosterAdded(UItemId),
    BoosterRemoved(UItemId),
    BoosterState(UItemId, bool),
    BoosterType(UItemId, AItemId),
    // Item - character
    CharacterAdded(UItemId),
    CharacterRemoved(UItemId),
    CharacterState(UItemId, bool),
    CharacterType(UItemId, AItemId),
    // Item - charge
    ChargeAdded(UItemId),
    ChargeRemoved(UItemId),
    ChargeState(UItemId, bool),
    ChargeType(UItemId, AItemId),
    // Item - drone
    DroneAdded(UItemId),
    DroneRemoved(UItemId),
    DroneMutationAdded(UItemId),
    DroneMutationAttr(UItemId),
    DroneMutationAttrs(UItemId),
    DroneMutationRemoved(UItemId, AItemId),
    DroneMutator(UItemId, AItemId),
    DroneNpcPropOverride(UItemId, Option<NpcProp>),
    DronePhysics(UItemId),
    DroneState(UItemId, MinionState),
    DroneType(UItemId, AItemId),
    // Item - fighter
    FighterAdded(UItemId),
    FighterRemoved(UItemId),
    FighterCountOverride(UItemId, Option<CountNz>),
    FighterPhysics(UItemId),
    FighterRearmMinionOverride(UItemId, Option<RearmMinion>),
    FighterState(UItemId, MinionState),
    FighterType(UItemId, AItemId),
    // Item - fit-wide effect
    FwEffectAdded(UItemId),
    FwEffectRemoved(UItemId),
    FwEffectState(UItemId, bool),
    FwEffectType(UItemId, AItemId),
    // Item - implant
    ImplantAdded(UItemId),
    ImplantRemoved(UItemId),
    ImplantState(UItemId, bool),
    ImplantType(UItemId, AItemId),
    // Item -module
    ModuleAdded(UItemId, RemoveMode),
    ModuleRemoved(UItemId, RemoveMode),
    ModuleMoved(UItemId, MoveMode),
    ModuleMutationAdded(UItemId),
    ModuleMutationAttr(UItemId),
    ModuleMutationAttrs(UItemId),
    ModuleMutationRemoved(UItemId, AItemId),
    ModuleMutator(UItemId, AItemId),
    ModuleOptionalReloadOverride(UItemId, Option<OptionalReload>),
    ModuleSpoolOverride(UItemId, Option<Spool>),
    ModuleState(UItemId, ModuleState),
    ModuleType(UItemId, AItemId),
    // Item - projected effect
    ProjEffectAdded(UItemId),
    ProjEffectRemoved(UItemId),
    ProjEffectState(UItemId, bool),
    ProjEffectType(UItemId, AItemId),
    // Item - rig
    RigAdded(UItemId),
    RigRemoved(UItemId),
    RigState(UItemId, bool),
    RigType(UItemId, AItemId),
    // Item - service
    ServiceAdded(UItemId),
    ServiceRemoved(UItemId),
    ServiceState(UItemId, ServiceState),
    ServiceType(UItemId, AItemId),
    // Item - ship
    ShipAdded(UItemId),
    ShipRemoved(UItemId),
    ShipPhysics(UItemId),
    ShipState(UItemId, bool),
    ShipType(UItemId, AItemId),
    // Item - skill
    SkillAdded(UItemId),
    SkillRemoved(UItemId),
    SkillLevel(UItemId, SkillLevel),
    SkillState(UItemId, bool),
    SkillType(UItemId, AItemId),
    // Item - stance
    StanceAdded(UItemId),
    StanceRemoved(UItemId),
    StanceState(UItemId, bool),
    StanceType(UItemId, AItemId),
    // Item - subsystem
    SubsystemAdded(UItemId),
    SubsystemRemoved(UItemId),
    SubsystemState(UItemId, bool),
    SubsystemType(UItemId, AItemId),
    // Item - system-wide effect
    SwEffectAdded(UItemId),
    SwEffectRemoved(UItemId),
    SwEffectState(UItemId, bool),
    SwEffectType(UItemId, AItemId),
}
