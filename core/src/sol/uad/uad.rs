use ordered_float::OrderedFloat as OF;

use crate::{
    sol::{
        DpsProfile, ItemKey, SecZone,
        uad::{fit::Fits, fleet::Fleets, item::Items},
    },
    src::Src,
    util::RSet,
};

// UAD stands for User and Adapted Data. Per definition, contains user-defined data, as well as some
// adapted data, stored on user-defined entities for optimization purposes.
#[derive(Clone)]
pub(in crate::sol) struct Uad {
    pub(in crate::sol) src: Src,
    pub(in crate::sol) fleets: Fleets,
    pub(in crate::sol) fits: Fits,
    pub(in crate::sol) sw_effects: RSet<ItemKey>,
    pub(in crate::sol) proj_effects: RSet<ItemKey>,
    pub(in crate::sol) items: Items,
    pub(in crate::sol) default_incoming_dps: DpsProfile,
    pub(in crate::sol) sec_zone: SecZone,
}
impl Uad {
    pub(in crate::sol) fn new(src: Src) -> Self {
        Self {
            src,
            fleets: Fleets::new(5),
            fits: Fits::new(50),
            sw_effects: RSet::new(),
            proj_effects: RSet::new(),
            items: Items::new(10000),
            default_incoming_dps: DpsProfile::try_new(OF(1.0), OF(1.0), OF(1.0), OF(1.0), None).unwrap(),
            sec_zone: SecZone::NullSec,
        }
    }
}
