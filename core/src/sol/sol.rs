use crate::{
    defs::SolItemId,
    sol::{fit::SolFits, fleet::SolFleets, item::SolItems, svc::SolSvcs, SolProjTracker},
    src::Src,
    util::StSet,
};

// Solar system glues everything together and is actual "god object" of the lib. It controls source
// which will be used for data and general item structure - including their kind, type IDs, which
// fit they belong to, which charges they have etc. But all the processing for those items (e.g.
// attribute calculation) happens in services, which are also stored on solar system, but are
// somewhat isolated.
#[derive(Clone)]
pub struct SolarSystem {
    pub(in crate::sol) src: Src,
    pub(in crate::sol) fleets: SolFleets,
    pub(in crate::sol) fits: SolFits,
    pub(in crate::sol) items: SolItems,
    pub(in crate::sol) sw_effects: StSet<SolItemId>,
    pub(in crate::sol) proj_effects: StSet<SolItemId>,
    pub(in crate::sol) proj_tracker: SolProjTracker,
    pub(in crate::sol) svcs: SolSvcs,
}
impl SolarSystem {
    pub fn new(src: Src) -> Self {
        Self {
            src,
            fleets: SolFleets::new(),
            fits: SolFits::new(),
            items: SolItems::new(),
            sw_effects: StSet::new(),
            proj_effects: StSet::new(),
            proj_tracker: SolProjTracker::new(),
            svcs: SolSvcs::new(),
        }
    }
}
