use crate::{
    err::basic::{ItemFoundError, ItemReceiveProjError},
    misc::{DpsProfile, NpcProp, OptionalReload, RearmMinion, Spool},
    num::PValue,
    ud::{ItemId, UData, UFit, UFitId, UItem, UItemId, UPhysics},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Some basic/uncategorized access methods
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UData {
    pub(crate) fn get_item_fit_ship_uid(&self, item_uid: UItemId) -> Option<UItemId> {
        let item = self.items.get(item_uid);
        let fit_uid = item.get_fit_uid()?;
        let fit = self.fits.get(fit_uid);
        fit.ship
    }
    pub(crate) fn get_charge_mult(&self, item_uid: UItemId) -> Option<PValue> {
        let cont_item_uid = match self.items.get(item_uid) {
            UItem::Autocharge(autocharge) => autocharge.get_cont_item_uid(),
            UItem::Charge(charge) => charge.get_cont_item_uid(),
            _ => return None,
        };
        match self.items.get(cont_item_uid) {
            UItem::Fighter(fighter) => fighter.get_count().map(|v| v.into_pvalue()),
            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Multi-tiered data fetchers (with priorities override > on-item/on-fit > on-sol)
////////////////////////////////////////////////////////////////////////////////////////////////////
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
    pub(crate) fn get_item_spool(&self, item_uid: UItemId, spool_override: Option<Spool>) -> Spool {
        match spool_override {
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
    pub(crate) fn get_item_npc_prop(&self, u_item: &UItem) -> Option<NpcProp> {
        u_item.get_npc_prop().map(|npc_prop| match npc_prop {
            Some(npc_prop) => npc_prop,
            None => self.default_npc_prop,
        })
    }
    pub(crate) fn get_item_optional_reload(
        &self,
        item_uid: UItemId,
        optional_reload_override: Option<OptionalReload>,
    ) -> OptionalReload {
        match optional_reload_override {
            Some(optional_reload) => optional_reload,
            None => {
                let u_item = self.items.get(item_uid);
                match u_item.get_optional_reload() {
                    Some(optional_reload) => optional_reload,
                    None => self.default_optional_reloads,
                }
            }
        }
    }
    pub(crate) fn get_item_rearm_minion(
        &self,
        item_uid: UItemId,
        rearm_minions_override: Option<RearmMinion>,
    ) -> RearmMinion {
        match rearm_minions_override {
            Some(rearm_minion) => rearm_minion,
            None => {
                let u_item = self.items.get(item_uid);
                match u_item.get_rearm_minion() {
                    Some(rearm_minion) => rearm_minion,
                    None => self.default_rearm_minions,
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Projection-related
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UData {
    pub(crate) fn get_fit_ship_radius(&self, fit_uid: UFitId) -> PValue {
        let Some(ship_uid) = self.fits.get(fit_uid).ship else {
            return PValue::ZERO;
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
    pub(crate) fn get_projectee_uid(&self, projectee_item_id: &ItemId) -> Result<UItemId, ProjecteeUidError> {
        let projectee_uid = self.items.iid_by_xid_err(projectee_item_id)?;
        let projectee_u_item = self.items.get(projectee_uid);
        if projectee_u_item.get_direct_physics().is_none() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.lib_get_name(),
            }
            .into());
        }
        Ok(projectee_uid)
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum ProjecteeUidError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
