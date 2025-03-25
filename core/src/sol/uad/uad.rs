use ordered_float::OrderedFloat as OF;

use crate::{
    sol::{
        DmgProfile, ItemId, SecZone,
        uad::{fit::Fits, fleet::Fleets, item::Items},
    },
    src::Src,
    util::StSet,
};

// UAD stands for User and Adapted Data. Per definition, contains user-defined data, as well as some
// adapted data, stored on user-defined entities for optimization purposes.
#[derive(Clone)]
pub(in crate::sol) struct Uad {
    pub(in crate::sol) src: Src,
    pub(in crate::sol) fleets: Fleets,
    pub(in crate::sol) fits: Fits,
    pub(in crate::sol) sw_effects: StSet<ItemId>,
    pub(in crate::sol) proj_effects: StSet<ItemId>,
    pub(in crate::sol) items: Items,
    pub(in crate::sol) default_incoming_dmg: DmgProfile,
    pub(in crate::sol) sec_zone: SecZone,
}
impl Uad {
    pub(in crate::sol) fn new(src: Src) -> Self {
        Self {
            src,
            fleets: Fleets::new(),
            fits: Fits::new(),
            sw_effects: StSet::new(),
            proj_effects: StSet::new(),
            items: Items::new(),
            default_incoming_dmg: DmgProfile::new(OF(1.0), OF(1.0), OF(1.0), OF(1.0)),
            sec_zone: SecZone::NullSec,
        }
    }
}
