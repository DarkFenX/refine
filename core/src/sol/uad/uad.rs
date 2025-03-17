use crate::{
    defs::{OF, SolItemId},
    sol::{
        SolDmgProfile, SolSecZone,
        uad::{fit::SolFits, fleet::SolFleets, item::SolItems},
    },
    src::Src,
    util::StSet,
};

// UAD stands for User and Adapted Data. Per definition, contains user-defined data, as well as some
// adapted data, stored on user-defined entities for optimization purposes.
#[derive(Clone)]
pub(in crate::sol) struct SolUad {
    pub(in crate::sol) src: Src,
    pub(in crate::sol) fleets: SolFleets,
    pub(in crate::sol) fits: SolFits,
    pub(in crate::sol) sw_effects: StSet<SolItemId>,
    pub(in crate::sol) proj_effects: StSet<SolItemId>,
    pub(in crate::sol) items: SolItems,
    pub(in crate::sol) default_incoming_dmg: SolDmgProfile,
    pub(in crate::sol) sec_zone: SolSecZone,
}
impl SolUad {
    pub(in crate::sol) fn new(src: Src) -> Self {
        Self {
            src,
            fleets: SolFleets::new(),
            fits: SolFits::new(),
            sw_effects: StSet::new(),
            proj_effects: StSet::new(),
            items: SolItems::new(),
            default_incoming_dmg: SolDmgProfile::new(OF(1.0), OF(1.0), OF(1.0), OF(1.0)),
            sec_zone: SolSecZone::NullSec,
        }
    }
}
