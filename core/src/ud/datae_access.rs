use crate::{
    misc::{DpsProfile, NpcProp, PValue, Spool},
    ud::{UData, UFit, UFitId, UItem, UItemId, UPhysics},
};

impl UData {
    pub(crate) fn get_fit_uid_rah_incoming_dps(&self, fit_uid: UFitId) -> DpsProfile {
        let fit = self.fits.get(fit_uid);
        self.get_fit_rah_incoming_dps(fit)
    }
    pub(crate) fn get_fit_rah_incoming_dps(&self, fit: &UFit) -> DpsProfile {
        match fit.rah_incoming_dps {
            Some(dps_profile) => dps_profile,
            None => self.default_incoming_dps,
        }
    }
    pub(crate) fn get_item_spool(&self, item_uid: UItemId, spool: Option<Spool>) -> Spool {
        match spool {
            Some(spool) => spool,
            None => {
                let u_item = self.items.get(item_uid);
                match u_item.get_spool() {
                    Some(spool) => spool,
                    None => self.default_spool,
                }
            }
        }
    }
    pub(crate) fn get_item_npc_prop(&self, item_uid: UItemId) -> Option<NpcProp> {
        let u_item = self.items.get(item_uid);
        u_item.get_npc_prop().map(|npc_prop| match npc_prop {
            Some(npc_prop) => npc_prop,
            None => self.default_npc_prop,
        })
    }
    pub(crate) fn get_item_reload_optionals(&self, item_uid: UItemId, reload_optionals: Option<bool>) -> bool {
        match reload_optionals {
            Some(reload_optionals) => reload_optionals,
            None => {
                let u_item = self.items.get(item_uid);
                match u_item.get_reload_optionals() {
                    Some(reload_optionals) => reload_optionals,
                    None => self.default_reload_optionals,
                }
            }
        }
    }
    pub(crate) fn get_item_rearm_minions(&self, item_uid: UItemId, rearm_minions: Option<bool>) -> bool {
        match rearm_minions {
            Some(rearm_minions) => rearm_minions,
            None => {
                let u_item = self.items.get(item_uid);
                match u_item.get_rearm_minions() {
                    Some(rearm_minions) => rearm_minions,
                    None => self.default_rearm_minions,
                }
            }
        }
    }
    // Projection-related
    pub(crate) fn get_fit_ship_radius(&self, fit_uid: UFitId) -> PValue {
        let ship_uid = match self.fits.get(fit_uid).ship {
            Some(ship_uid) => ship_uid,
            None => return PValue::ZERO,
        };
        self.items.get(ship_uid).get_direct_radius()
    }
    pub(crate) fn get_fit_ship_physics(&self, fit_uid: UFitId) -> UPhysics {
        let fit = self.fits.get(fit_uid);
        match fit.ship {
            Some(ship_uid) => *self.items.get(ship_uid).dc_ship().unwrap().get_physics(),
            None => UPhysics::default(),
        }
    }
    pub(crate) fn get_physics_carrier(&self, item_uid: UItemId) -> Option<UItemId> {
        match self.items.get(item_uid) {
            UItem::Autocharge(autocharge) => self.get_physics_carrier(autocharge.get_cont_item_uid()),
            UItem::Booster(booster) => self.fits.get(booster.get_fit_uid()).ship,
            UItem::Character(character) => self.fits.get(character.get_fit_uid()).ship,
            UItem::Charge(charge) => self.get_physics_carrier(charge.get_cont_item_uid()),
            UItem::Drone(_) => Some(item_uid),
            UItem::Fighter(_) => Some(item_uid),
            UItem::FwEffect(_) => None,
            UItem::Implant(implant) => self.fits.get(implant.get_fit_uid()).ship,
            UItem::Module(module) => self.fits.get(module.get_fit_uid()).ship,
            UItem::ProjEffect(_) => None,
            UItem::Service(service) => self.fits.get(service.get_fit_uid()).ship,
            UItem::Rig(rig) => self.fits.get(rig.get_fit_uid()).ship,
            UItem::Ship(_) => Some(item_uid),
            UItem::Skill(skill) => self.fits.get(skill.get_fit_uid()).ship,
            UItem::Stance(stance) => self.fits.get(stance.get_fit_uid()).ship,
            UItem::Subsystem(subsystem) => self.fits.get(subsystem.get_fit_uid()).ship,
            UItem::SwEffect(_) => None,
        }
    }
}
