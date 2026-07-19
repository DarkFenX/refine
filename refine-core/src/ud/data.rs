use crate::{
    misc::{DpsProfile, NpcProp, OptionalReload, RearmMinion, SecZone, Spool},
    num::{PValue, UnitInterval},
    rd::RcData,
    ud::{UFits, UFleets, UItemId, UItems},
    util::RSet,
};

// UAD stands for User and Adapted Data. Per definition, contains user-defined data, as well as
// container with EVE-derived-data, since which one is used is also chosen by user.
#[derive(Clone)]
pub(crate) struct UData {
    pub(crate) r_data: RcData,
    pub(crate) fleets: UFleets,
    pub(crate) fits: UFits,
    pub(crate) sw_effects: RSet<UItemId>,
    pub(crate) proj_effects: RSet<UItemId>,
    pub(crate) items: UItems,
    pub(crate) sec_zone: SecZone,
    // Default setting used in stats / RAH sim
    pub(crate) default_incoming_dps: DpsProfile,
    // Default settings related to item cycles
    pub(crate) default_spool: Spool,
    pub(crate) default_npc_prop: NpcProp,
    pub(crate) default_optional_reloads: OptionalReload,
    pub(crate) default_rearm_minions: RearmMinion,
}
impl UData {
    pub(crate) fn new(r_data: RcData) -> Self {
        Self {
            r_data,
            fleets: UFleets::new(5),
            fits: UFits::new(50),
            sw_effects: RSet::new(),
            proj_effects: RSet::new(),
            items: UItems::new(10000),
            sec_zone: SecZone::NullSec,
            default_incoming_dps: DpsProfile {
                em: PValue::ONE,
                thermal: PValue::ONE,
                kinetic: PValue::ONE,
                explosive: PValue::ONE,
                ..
            },
            default_spool: Spool::SpoolScale(UnitInterval::from_f64_clamped(1.0)),
            default_npc_prop: NpcProp::Chase,
            default_optional_reloads: OptionalReload::OnEmpty,
            default_rearm_minions: RearmMinion::Disabled,
        }
    }
}
