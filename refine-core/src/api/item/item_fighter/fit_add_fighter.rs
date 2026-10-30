use itertools::Itertools;

use crate::{
    ad::AItemId,
    api::{Coordinates, FighterMut, FitMut, ItemTypeId, MinionState, Movement},
    sol::{SolarSystem, UndoStep},
    ud::{UFighter, UFitId, UItem, UItemId, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_fighter(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        state: MinionState,
        physics: UPhysics,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        let item_id = self.u_data.items.alloc_id();
        let u_fighter = UFighter::new(item_id, type_aid, fit_uid, state, physics, &self.u_data.r_data);
        let u_item = UItem::Fighter(u_fighter);
        let fighter_uid = self.u_data.items.add(u_item);
        u_fit.fighters.insert(fighter_uid);
        // Add fighter and autocharges to services
        SolarSystem::util_add_fighter_with_acs(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            &mut self.cache.eupdates,
        );
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FighterAdded(fighter_uid));
        }
        fighter_uid
    }
    pub(in crate::api) fn internal_reattach_fighter(&mut self, fighter_uid: UItemId) {
        self.u_data.items.reattach(fighter_uid);
        let u_fighter = self.u_data.items.get(fighter_uid).dc_fighter().unwrap();
        let fit_uid = u_fighter.get_fit_uid();
        let autocharge_uids = u_fighter.get_autocharges().values().collect_vec();
        for autocharge_uid in autocharge_uids {
            self.u_data.items.reattach(autocharge_uid);
        }
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.fighters.insert(fighter_uid);
        // Add fighter and its existing autocharges to services
        SolarSystem::util_add_fighter(&mut self.u_data, &mut self.svc, fighter_uid, &mut self.cache.eupdates);
        SolarSystem::util_reg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, fighter_uid);
        SolarSystem::util_reattach_item_autocharges(
            &mut self.u_data,
            &mut self.svc,
            &mut self.rev_projs,
            fighter_uid,
            &mut self.cache.eupdates,
        );
    }
}

impl<'s> FitMut<'s> {
    pub fn add_fighter(
        &mut self,
        type_id: ItemTypeId,
        state: MinionState,
        coordinates: Option<Coordinates>,
        movement: Option<Movement>,
    ) -> FighterMut<'_> {
        let mut u_physics = UPhysics::default();
        if let Some(coordinates) = coordinates {
            u_physics.coordinates = coordinates.into_xyz();
        }
        if let Some(movement) = movement {
            u_physics.direction = movement.direction.into_xyz();
            u_physics.speed = movement.speed;
        }
        let fighter_uid = self
            .sol
            .internal_add_fighter(self.uid, type_id.into_aid(), state, u_physics);
        FighterMut::new(self.sol, fighter_uid)
    }
}
