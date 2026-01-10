use crate::{
    num::PValue,
    rd::Src,
    sol::SolarSystem,
    ud::{UData, UEffectUpdates, UItem, UItemId, UShipKind},
};

struct ItemUIds {
    boosters: Vec<UItemId>,
    characters: Vec<UItemId>,
    charges: Vec<UItemId>,
    drones: Vec<UItemId>,
    fighters: Vec<UItemId>,
    fw_effects: Vec<UItemId>,
    implants: Vec<UItemId>,
    modules: Vec<UItemId>,
    proj_effects: Vec<UItemId>,
    services: Vec<UItemId>,
    rigs: Vec<UItemId>,
    ships: Vec<UItemId>,
    skills: Vec<UItemId>,
    stances: Vec<UItemId>,
    subsystems: Vec<UItemId>,
    sw_effects: Vec<UItemId>,
}
impl ItemUIds {
    fn from_u_data(u_data: &UData) -> Self {
        let mut data = Self {
            boosters: Vec::new(),
            characters: Vec::new(),
            charges: Vec::new(),
            drones: Vec::new(),
            fighters: Vec::new(),
            fw_effects: Vec::new(),
            implants: Vec::new(),
            modules: Vec::new(),
            proj_effects: Vec::new(),
            services: Vec::new(),
            rigs: Vec::new(),
            ships: Vec::new(),
            skills: Vec::new(),
            stances: Vec::new(),
            subsystems: Vec::new(),
            sw_effects: Vec::new(),
        };
        for (item_uid, u_item) in u_data.items.iter() {
            match u_item {
                // Autocharges are added/removed by whichever item is carrying them (e.g. fighter)
                UItem::Autocharge(_) => (),
                UItem::Booster(_) => data.boosters.push(item_uid),
                UItem::Character(_) => data.characters.push(item_uid),
                UItem::Charge(_) => data.charges.push(item_uid),
                UItem::Drone(_) => data.drones.push(item_uid),
                UItem::Fighter(_) => data.fighters.push(item_uid),
                UItem::FwEffect(_) => data.fw_effects.push(item_uid),
                UItem::Implant(_) => data.implants.push(item_uid),
                UItem::Module(_) => data.modules.push(item_uid),
                UItem::ProjEffect(_) => data.proj_effects.push(item_uid),
                UItem::Service(_) => data.services.push(item_uid),
                UItem::Rig(_) => data.rigs.push(item_uid),
                UItem::Ship(_) => data.ships.push(item_uid),
                UItem::Skill(_) => data.skills.push(item_uid),
                UItem::Stance(_) => data.stances.push(item_uid),
                UItem::Subsystem(_) => data.subsystems.push(item_uid),
                UItem::SwEffect(_) => data.sw_effects.push(item_uid),
            }
        }
        data
    }
}

impl SolarSystem {
    pub fn set_src(&mut self, mut src: Src) {
        let item_uids = ItemUIds::from_u_data(&self.u_data);
        let mut reuse_eupdates = UEffectUpdates::new();
        self.unload_items(&item_uids, &mut reuse_eupdates);
        // Set new source
        std::mem::swap(&mut self.u_data.src, &mut src);
        for item in self.u_data.items.values_mut() {
            item.src_changed(&self.u_data.src);
        }
        // Update fit kind
        for fit in self.u_data.fits.values_mut() {
            fit.ship_kind = match fit.ship {
                Some(ship_uid) => self.u_data.items.get(ship_uid).dc_ship().unwrap().get_kind(),
                None => UShipKind::Unknown,
            }
        }
        // Update on-projection data due to changed item radii
        self.update_projections();
        self.load_items(&item_uids, &mut reuse_eupdates);
    }
    fn unload_items(&mut self, item_uids: &ItemUIds, reuse_eupdates: &mut UEffectUpdates) {
        for &booster_uid in item_uids.boosters.iter() {
            SolarSystem::util_remove_booster(&mut self.u_data, &mut self.svc, booster_uid, reuse_eupdates);
        }
        for &character_uid in item_uids.characters.iter() {
            SolarSystem::util_remove_character(&mut self.u_data, &mut self.svc, character_uid, reuse_eupdates);
        }
        for &charge_uid in item_uids.charges.iter() {
            SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_uid, reuse_eupdates);
        }
        for &drone_uid in item_uids.drones.iter() {
            SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_uid, reuse_eupdates);
        }
        for &fighter_uid in item_uids.fighters.iter() {
            SolarSystem::util_remove_fighter_with_acs(
                &mut self.u_data,
                &mut self.svc,
                &mut self.rev_projs,
                fighter_uid,
                reuse_eupdates,
            );
        }
        for &fw_effect_uid in item_uids.fw_effects.iter() {
            SolarSystem::util_remove_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_uid, reuse_eupdates);
        }
        for &implant_uid in item_uids.implants.iter() {
            SolarSystem::util_remove_implant(&mut self.u_data, &mut self.svc, implant_uid, reuse_eupdates);
        }
        for &module_uid in item_uids.modules.iter() {
            SolarSystem::util_remove_module_with_charge_act(
                &mut self.u_data,
                &mut self.svc,
                module_uid,
                reuse_eupdates,
            );
        }
        for &proj_effect_uid in item_uids.proj_effects.iter() {
            SolarSystem::util_remove_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_uid, reuse_eupdates);
        }
        for &service_uid in item_uids.services.iter() {
            SolarSystem::util_remove_service(&mut self.u_data, &mut self.svc, service_uid, reuse_eupdates);
        }
        for &rig_uid in item_uids.rigs.iter() {
            SolarSystem::util_remove_rig(&mut self.u_data, &mut self.svc, rig_uid, reuse_eupdates);
        }
        for &ship_uid in item_uids.ships.iter() {
            SolarSystem::util_remove_ship(&mut self.u_data, &mut self.svc, ship_uid, reuse_eupdates);
        }
        for &skill_uid in item_uids.skills.iter() {
            SolarSystem::util_remove_skill(&mut self.u_data, &mut self.svc, skill_uid, reuse_eupdates);
        }
        for &stance_uid in item_uids.stances.iter() {
            SolarSystem::util_remove_stance(&mut self.u_data, &mut self.svc, stance_uid, reuse_eupdates);
        }
        for &subsystem_uid in item_uids.subsystems.iter() {
            SolarSystem::util_remove_subsystem(&mut self.u_data, &mut self.svc, subsystem_uid, reuse_eupdates);
        }
        for &sw_effect_uid in item_uids.sw_effects.iter() {
            SolarSystem::util_remove_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_uid, reuse_eupdates);
        }
    }
    fn load_items(&mut self, item_uids: &ItemUIds, reuse_eupdates: &mut UEffectUpdates) {
        for &booster_uid in item_uids.boosters.iter() {
            SolarSystem::util_add_booster(&mut self.u_data, &mut self.svc, booster_uid, reuse_eupdates);
        }
        for &character_uid in item_uids.characters.iter() {
            SolarSystem::util_add_character(&mut self.u_data, &mut self.svc, character_uid, reuse_eupdates);
        }
        for &charge_uid in item_uids.charges.iter() {
            SolarSystem::util_add_charge(&mut self.u_data, &mut self.svc, charge_uid, reuse_eupdates);
        }
        for &drone_uid in item_uids.drones.iter() {
            SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, reuse_eupdates);
        }
        for &fighter_uid in item_uids.fighters.iter() {
            SolarSystem::util_add_fighter_with_acs(
                &mut self.u_data,
                &mut self.svc,
                &mut self.rev_projs,
                fighter_uid,
                reuse_eupdates,
            );
        }
        for &fw_effect_uid in item_uids.fw_effects.iter() {
            SolarSystem::util_add_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_uid, reuse_eupdates);
        }
        for &implant_uid in item_uids.implants.iter() {
            SolarSystem::util_add_implant(&mut self.u_data, &mut self.svc, implant_uid, reuse_eupdates);
        }
        for &module_uid in item_uids.modules.iter() {
            SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_uid, reuse_eupdates);
        }
        for &proj_effect_uid in item_uids.proj_effects.iter() {
            SolarSystem::util_add_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_uid, reuse_eupdates);
        }
        for &service_uid in item_uids.services.iter() {
            SolarSystem::util_add_service(&mut self.u_data, &mut self.svc, service_uid, reuse_eupdates);
        }
        for &rig_uid in item_uids.rigs.iter() {
            SolarSystem::util_add_rig(&mut self.u_data, &mut self.svc, rig_uid, reuse_eupdates);
        }
        for &ship_uid in item_uids.ships.iter() {
            SolarSystem::util_add_ship(&mut self.u_data, &mut self.svc, ship_uid, reuse_eupdates);
        }
        for &skill_uid in item_uids.skills.iter() {
            SolarSystem::util_add_skill(&mut self.u_data, &mut self.svc, skill_uid, reuse_eupdates);
        }
        for &stance_uid in item_uids.stances.iter() {
            SolarSystem::util_add_stance(&mut self.u_data, &mut self.svc, stance_uid, reuse_eupdates);
        }
        for &subsystem_uid in item_uids.subsystems.iter() {
            SolarSystem::util_add_subsystem(&mut self.u_data, &mut self.svc, subsystem_uid, reuse_eupdates);
        }
        for &sw_effect_uid in item_uids.sw_effects.iter() {
            SolarSystem::util_add_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_uid, reuse_eupdates);
        }
    }
    fn update_projections(&mut self) {
        let mut projection_updates = Vec::new();
        for (fit_uid, u_fit) in self.u_data.fits.iter() {
            let ship_radius = self.u_data.get_fit_ship_radius(fit_uid);
            for module_uid in u_fit.iter_module_uids() {
                record_projection(&mut projection_updates, &self.u_data, module_uid, ship_radius);
                let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
                if let Some(charge_uid) = u_module.get_charge_uid() {
                    record_projection(&mut projection_updates, &self.u_data, charge_uid, ship_radius);
                }
            }
            for &drone_uid in u_fit.drones.iter() {
                let drone_radius = self.u_data.items.get(drone_uid).get_direct_radius();
                record_projection(&mut projection_updates, &self.u_data, drone_uid, drone_radius);
            }
            for &fighter_uid in u_fit.fighters.iter() {
                let fighter_radius = self.u_data.items.get(fighter_uid).get_direct_radius();
                record_projection(&mut projection_updates, &self.u_data, fighter_uid, fighter_radius);
            }
        }
        for (projector_uid, projectee_uid, src_rad, tgt_rag) in projection_updates {
            let projector_u_item = self.u_data.items.get_mut(projector_uid);
            projector_u_item
                .get_projs_mut()
                .unwrap()
                .get_proj_data_mut(&projectee_uid)
                .unwrap()
                .update_radii(src_rad, tgt_rag);
        }
    }
}

fn record_projection(
    projection_updates: &mut Vec<(UItemId, UItemId, PValue, PValue)>,
    u_data: &UData,
    item_uid: UItemId,
    src_rad: PValue,
) {
    let u_item = u_data.items.get(item_uid);
    for (projectee_uid, _u_proj_data) in u_item.get_projs().unwrap().iter_projectees_and_datas() {
        let projectee_radius = u_data.items.get(projectee_uid).get_direct_radius();
        projection_updates.push((item_uid, projectee_uid, src_rad, projectee_radius));
    }
}
