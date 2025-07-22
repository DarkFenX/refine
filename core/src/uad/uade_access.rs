use crate::{
    def::{AttrVal, OF},
    misc::{DpsProfile, Spool},
    uad::{Uad, UadFit, UadFitKey, UadItemKey},
};

impl Uad {
    pub(crate) fn get_fit_key_rah_incoming_dps(&self, fit_key: UadFitKey) -> DpsProfile {
        let fit = self.fits.get(fit_key);
        self.get_fit_rah_incoming_dps(fit)
    }
    pub(crate) fn get_fit_rah_incoming_dps(&self, fit: &UadFit) -> DpsProfile {
        match fit.rah_incoming_dps {
            Some(dps_profile) => dps_profile,
            None => self.default_incoming_dps,
        }
    }
    pub(crate) fn get_item_key_spool(&self, item_key: UadItemKey, spool: Option<Spool>) -> Spool {
        match spool {
            Some(spool) => spool,
            None => {
                let uad_item = self.items.get(item_key);
                match uad_item.get_spool() {
                    Some(spool) => spool,
                    None => self.default_spool,
                }
            }
        }
    }
    pub(crate) fn get_item_radius(&self, item_key: UadItemKey) -> AttrVal {
        match self.items.get(item_key).get_a_xt() {
            Some(a_xt) => a_xt.radius,
            None => OF(0.0),
        }
    }
    pub(crate) fn get_ship_radius_by_fit_key(&self, fit_key: UadFitKey) -> AttrVal {
        let ship_key = match self.fits.get(fit_key).ship {
            Some(ship_key) => ship_key,
            None => return OF(0.0),
        };
        self.get_item_radius(ship_key)
    }
}
