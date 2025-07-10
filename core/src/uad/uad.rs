use crate::{
    def::{AttrVal, FitKey, ItemKey, OF},
    misc::{DpsProfile, SecZone, Spool, UnitInterval},
    src::Src,
    uad::{fit::Fits, fleet::Fleets, item::Items},
    util::RSet,
};

// UAD stands for User and Adapted Data. Per definition, contains user-defined data, as well as some
// adapted data, stored on user-defined entities for optimization purposes.
#[derive(Clone)]
pub(crate) struct Uad {
    pub(crate) src: Src,
    pub(crate) fleets: Fleets,
    pub(crate) fits: Fits,
    pub(crate) sw_effects: RSet<ItemKey>,
    pub(crate) proj_effects: RSet<ItemKey>,
    pub(crate) items: Items,
    pub(crate) default_incoming_dps: DpsProfile,
    pub(crate) default_spool: Spool,
    pub(crate) sec_zone: SecZone,
}
impl Uad {
    pub(crate) fn new(src: Src) -> Self {
        Self {
            src,
            fleets: Fleets::new(5),
            fits: Fits::new(50),
            sw_effects: RSet::new(),
            proj_effects: RSet::new(),
            items: Items::new(10000),
            default_incoming_dps: DpsProfile::try_new(OF(1.0), OF(1.0), OF(1.0), OF(1.0), None).unwrap(),
            default_spool: Spool::SpoolScale(UnitInterval::new_clamped_of64(OF(1.0))),
            sec_zone: SecZone::NullSec,
        }
    }
    pub(crate) fn get_item_radius(&self, item_key: ItemKey) -> AttrVal {
        match self.items.get(item_key).get_a_xt() {
            Some(a_xt) => a_xt.radius,
            None => OF(0.0),
        }
    }
    pub(crate) fn get_ship_radius_by_fit_key(&self, fit_key: FitKey) -> AttrVal {
        let ship_key = match self.fits.get(fit_key).ship {
            Some(ship_key) => ship_key,
            None => return OF(0.0),
        };
        self.get_item_radius(ship_key)
    }
}
