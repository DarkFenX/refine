use crate::{
    PValue,
    stats::{StatBrFallibleError, StatDmg, StatMining, StatOutReps, StatResult, err::StatFleetAppliedError},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct FleetStats {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub dmg: StatResult<StatDmg, !, StatBrFallibleError<StatFleetAppliedError>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mps: StatResult<StatMining, !, !> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_nps: StatResult<PValue, !, StatBrFallibleError<StatFleetAppliedError>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_rps: StatResult<StatOutReps, !, StatBrFallibleError<StatFleetAppliedError>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub outgoing_cps: StatResult<PValue, !, StatBrFallibleError<StatFleetAppliedError>> = StatResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "StatResult::is_not_requested"))]
    pub mass: StatResult<PValue, !, !> = StatResult::NotRequested,
}
