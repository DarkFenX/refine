use crate::{
    misc::{DpsProfile, SecZone, Spool},
    rd::Src,
    ud::{UFits, UFleets, UItemId, UItems},
    util::RSet,
};

// UAD stands for User and Adapted Data. Per definition, contains user-defined data, as well as some
// adapted data, stored on user-defined entities for optimization purposes.
#[derive(Clone)]
pub(crate) struct UData {
    pub(crate) src: Src,
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
    pub(crate) default_reload_optionals: bool,
    pub(crate) default_rearm_minions: bool,
}
impl UData {
    pub(crate) fn new(src: Src) -> Self {
        Self {
            src,
            fleets: UFleets::new(5),
            fits: UFits::new(50),
            sw_effects: RSet::new(),
            proj_effects: RSet::new(),
            items: UItems::new(10000),
            sec_zone: SecZone::NullSec,
            default_incoming_dps: DpsProfile::try_new(OF(1.0), OF(1.0), OF(1.0), OF(1.0), None).unwrap(),
            default_spool: Spool::SpoolScale(UnitInterval::new_clamped_of64(OF(1.0))),
            default_reload_optionals: true,
            default_rearm_minions: false,
        }
    }
}
