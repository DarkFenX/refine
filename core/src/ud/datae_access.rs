use crate::{
    def::{AttrVal, OF},
    misc::{DpsProfile, Spool},
    ud::{UData, UFit, UFitKey, UItemKey, UPosition},
};

impl UData {
    pub(crate) fn get_fit_key_rah_incoming_dps(&self, fit_key: UFitKey) -> DpsProfile {
        let fit = self.fits.get(fit_key);
        self.get_fit_rah_incoming_dps(fit)
    }
    pub(crate) fn get_fit_rah_incoming_dps(&self, fit: &UFit) -> DpsProfile {
        match fit.rah_incoming_dps {
            Some(dps_profile) => dps_profile,
            None => self.default_incoming_dps,
        }
    }
    pub(crate) fn get_item_key_spool(&self, item_key: UItemKey, spool: Option<Spool>) -> Spool {
        match spool {
            Some(spool) => spool,
            None => {
                let u_item = self.items.get(item_key);
                match u_item.get_spool() {
                    Some(spool) => spool,
                    None => self.default_spool,
                }
            }
        }
    }
    pub(crate) fn get_item_radius(&self, item_key: UItemKey) -> AttrVal {
        match self.items.get(item_key).get_axt() {
            Some(axt) => axt.radius,
            None => OF(0.0),
        }
    }
    pub(crate) fn get_ship_radius_by_fit_key(&self, fit_key: UFitKey) -> AttrVal {
        let ship_key = match self.fits.get(fit_key).ship {
            Some(ship_key) => ship_key,
            None => return OF(0.0),
        };
        self.get_item_radius(ship_key)
    }
    pub(crate) fn get_ship_pos_by_fit_key(&self, fit_key: UFitKey) -> UPosition {
        let fit = self.fits.get(fit_key);
        match fit.ship {
            Some(ship_key) => *self.items.get(ship_key).get_ship().unwrap().get_pos(),
            None => UPosition::default(),
        }
    }
}
