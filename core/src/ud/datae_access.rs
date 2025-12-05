use crate::{
    def::{AttrVal, OF},
    misc::{DpsProfile, Spool},
    ud::{UData, UFit, UFitKey, UItem, UItemKey, UPhysics},
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
    // Projection-related
    pub(crate) fn get_ship_radius_by_fit_key(&self, fit_key: UFitKey) -> AttrVal {
        let ship_key = match self.fits.get(fit_key).ship {
            Some(ship_key) => ship_key,
            None => return OF(0.0),
        };
        self.items.get(ship_key).get_direct_radius()
    }
    pub(crate) fn get_ship_physics_by_fit_key(&self, fit_key: UFitKey) -> UPhysics {
        let fit = self.fits.get(fit_key);
        match fit.ship {
            Some(ship_key) => *self.items.get(ship_key).dc_ship().unwrap().get_physics(),
            None => UPhysics::default(),
        }
    }
    pub(crate) fn get_physics_carrier_key(&self, item_key: UItemKey) -> Option<UItemKey> {
        match self.items.get(item_key) {
            UItem::Autocharge(autocharge) => self.get_physics_carrier_key(autocharge.get_cont_item_key()),
            UItem::Booster(booster) => self.fits.get(booster.get_fit_key()).ship,
            UItem::Character(character) => self.fits.get(character.get_fit_key()).ship,
            UItem::Charge(charge) => self.get_physics_carrier_key(charge.get_cont_item_key()),
            UItem::Drone(_) => Some(item_key),
            UItem::Fighter(_) => Some(item_key),
            UItem::FwEffect(_) => None,
            UItem::Implant(implant) => self.fits.get(implant.get_fit_key()).ship,
            UItem::Module(module) => self.fits.get(module.get_fit_key()).ship,
            UItem::ProjEffect(_) => None,
            UItem::Service(service) => self.fits.get(service.get_fit_key()).ship,
            UItem::Rig(rig) => self.fits.get(rig.get_fit_key()).ship,
            UItem::Ship(_) => Some(item_key),
            UItem::Skill(skill) => self.fits.get(skill.get_fit_key()).ship,
            UItem::Stance(stance) => self.fits.get(stance.get_fit_key()).ship,
            UItem::Subsystem(subsystem) => self.fits.get(subsystem.get_fit_key()).ship,
            UItem::SwEffect(_) => None,
        }
    }
}
