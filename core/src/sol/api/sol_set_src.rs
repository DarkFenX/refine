use crate::{
    def::AttrVal,
    sol::SolarSystem,
    src::Src,
    ud::{UData, UEffectUpdates, UItem, UItemKey, UShipKind},
};

struct ItemKeys {
    boosters: Vec<UItemKey>,
    characters: Vec<UItemKey>,
    charges: Vec<UItemKey>,
    drones: Vec<UItemKey>,
    fighters: Vec<UItemKey>,
    fw_effects: Vec<UItemKey>,
    implants: Vec<UItemKey>,
    modules: Vec<UItemKey>,
    proj_effects: Vec<UItemKey>,
    services: Vec<UItemKey>,
    rigs: Vec<UItemKey>,
    ships: Vec<UItemKey>,
    skills: Vec<UItemKey>,
    stances: Vec<UItemKey>,
    subsystems: Vec<UItemKey>,
    sw_effects: Vec<UItemKey>,
}
impl ItemKeys {
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
        for (item_key, u_item) in u_data.items.iter() {
            match u_item {
                // Autocharges are added/removed by whichever item is carrying them (e.g. fighter)
                UItem::Autocharge(_) => (),
                UItem::Booster(_) => data.boosters.push(item_key),
                UItem::Character(_) => data.characters.push(item_key),
                UItem::Charge(_) => data.charges.push(item_key),
                UItem::Drone(_) => data.drones.push(item_key),
                UItem::Fighter(_) => data.fighters.push(item_key),
                UItem::FwEffect(_) => data.fw_effects.push(item_key),
                UItem::Implant(_) => data.implants.push(item_key),
                UItem::Module(_) => data.modules.push(item_key),
                UItem::ProjEffect(_) => data.proj_effects.push(item_key),
                UItem::Service(_) => data.services.push(item_key),
                UItem::Rig(_) => data.rigs.push(item_key),
                UItem::Ship(_) => data.ships.push(item_key),
                UItem::Skill(_) => data.skills.push(item_key),
                UItem::Stance(_) => data.stances.push(item_key),
                UItem::Subsystem(_) => data.subsystems.push(item_key),
                UItem::SwEffect(_) => data.sw_effects.push(item_key),
            }
        }
        data
    }
}

impl SolarSystem {
    pub fn set_src(&mut self, mut src: Src) {
        let item_keys = ItemKeys::from_u_data(&self.u_data);
        let mut reuse_eupdates = UEffectUpdates::new();
        self.unload_items(&item_keys, &mut reuse_eupdates);
        // Set new source
        std::mem::swap(&mut self.u_data.src, &mut src);
        for item in self.u_data.items.values_mut() {
            item.src_changed(&self.u_data.src);
        }
        // Update fit kind
        for fit in self.u_data.fits.values_mut() {
            fit.ship_kind = match fit.ship {
                Some(ship_key) => self.u_data.items.get(ship_key).dc_ship().unwrap().get_kind(),
                None => UShipKind::Unknown,
            }
        }
        // Update on-projection data due to changed item radii
        self.update_projections();
        self.load_items(&item_keys, &mut reuse_eupdates);
    }
    fn unload_items(&mut self, item_keys: &ItemKeys, reuse_eupdates: &mut UEffectUpdates) {
        for &booster_key in item_keys.boosters.iter() {
            SolarSystem::util_remove_booster(&mut self.u_data, &mut self.svc, booster_key, reuse_eupdates);
        }
        for &character_key in item_keys.characters.iter() {
            SolarSystem::util_remove_character(&mut self.u_data, &mut self.svc, character_key, reuse_eupdates);
        }
        for &charge_key in item_keys.charges.iter() {
            SolarSystem::util_remove_charge(&mut self.u_data, &mut self.svc, charge_key, reuse_eupdates);
        }
        for &drone_key in item_keys.drones.iter() {
            SolarSystem::util_remove_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        }
        for &fighter_key in item_keys.fighters.iter() {
            SolarSystem::util_remove_fighter_with_acs(
                &mut self.u_data,
                &mut self.svc,
                &mut self.rev_projs,
                fighter_key,
                reuse_eupdates,
            );
        }
        for &fw_effect_key in item_keys.fw_effects.iter() {
            SolarSystem::util_remove_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_key, reuse_eupdates);
        }
        for &implant_key in item_keys.implants.iter() {
            SolarSystem::util_remove_implant(&mut self.u_data, &mut self.svc, implant_key, reuse_eupdates);
        }
        for &module_key in item_keys.modules.iter() {
            SolarSystem::util_remove_module_with_charge_act(
                &mut self.u_data,
                &mut self.svc,
                module_key,
                reuse_eupdates,
            );
        }
        for &proj_effect_key in item_keys.proj_effects.iter() {
            SolarSystem::util_remove_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_key, reuse_eupdates);
        }
        for &service_key in item_keys.services.iter() {
            SolarSystem::util_remove_service(&mut self.u_data, &mut self.svc, service_key, reuse_eupdates);
        }
        for &rig_key in item_keys.rigs.iter() {
            SolarSystem::util_remove_rig(&mut self.u_data, &mut self.svc, rig_key, reuse_eupdates);
        }
        for &ship_key in item_keys.ships.iter() {
            SolarSystem::util_remove_ship(&mut self.u_data, &mut self.svc, ship_key, reuse_eupdates);
        }
        for &skill_key in item_keys.skills.iter() {
            SolarSystem::util_remove_skill(&mut self.u_data, &mut self.svc, skill_key, reuse_eupdates);
        }
        for &stance_key in item_keys.stances.iter() {
            SolarSystem::util_remove_stance(&mut self.u_data, &mut self.svc, stance_key, reuse_eupdates);
        }
        for &subsystem_key in item_keys.subsystems.iter() {
            SolarSystem::util_remove_subsystem(&mut self.u_data, &mut self.svc, subsystem_key, reuse_eupdates);
        }
        for &sw_effect_key in item_keys.sw_effects.iter() {
            SolarSystem::util_remove_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_key, reuse_eupdates);
        }
    }
    fn load_items(&mut self, item_keys: &ItemKeys, reuse_eupdates: &mut UEffectUpdates) {
        for &booster_key in item_keys.boosters.iter() {
            SolarSystem::util_add_booster(&mut self.u_data, &mut self.svc, booster_key, reuse_eupdates);
        }
        for &character_key in item_keys.characters.iter() {
            SolarSystem::util_add_character(&mut self.u_data, &mut self.svc, character_key, reuse_eupdates);
        }
        for &charge_key in item_keys.charges.iter() {
            SolarSystem::util_add_charge(&mut self.u_data, &mut self.svc, charge_key, reuse_eupdates);
        }
        for &drone_key in item_keys.drones.iter() {
            SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_key, reuse_eupdates);
        }
        for &fighter_key in item_keys.fighters.iter() {
            SolarSystem::util_add_fighter_with_acs(
                &mut self.u_data,
                &mut self.svc,
                &mut self.rev_projs,
                fighter_key,
                reuse_eupdates,
            );
        }
        for &fw_effect_key in item_keys.fw_effects.iter() {
            SolarSystem::util_add_fw_effect(&mut self.u_data, &mut self.svc, fw_effect_key, reuse_eupdates);
        }
        for &implant_key in item_keys.implants.iter() {
            SolarSystem::util_add_implant(&mut self.u_data, &mut self.svc, implant_key, reuse_eupdates);
        }
        for &module_key in item_keys.modules.iter() {
            SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
        }
        for &proj_effect_key in item_keys.proj_effects.iter() {
            SolarSystem::util_add_proj_effect(&mut self.u_data, &mut self.svc, proj_effect_key, reuse_eupdates);
        }
        for &service_key in item_keys.services.iter() {
            SolarSystem::util_add_service(&mut self.u_data, &mut self.svc, service_key, reuse_eupdates);
        }
        for &rig_key in item_keys.rigs.iter() {
            SolarSystem::util_add_rig(&mut self.u_data, &mut self.svc, rig_key, reuse_eupdates);
        }
        for &ship_key in item_keys.ships.iter() {
            SolarSystem::util_add_ship(&mut self.u_data, &mut self.svc, ship_key, reuse_eupdates);
        }
        for &skill_key in item_keys.skills.iter() {
            SolarSystem::util_add_skill(&mut self.u_data, &mut self.svc, skill_key, reuse_eupdates);
        }
        for &stance_key in item_keys.stances.iter() {
            SolarSystem::util_add_stance(&mut self.u_data, &mut self.svc, stance_key, reuse_eupdates);
        }
        for &subsystem_key in item_keys.subsystems.iter() {
            SolarSystem::util_add_subsystem(&mut self.u_data, &mut self.svc, subsystem_key, reuse_eupdates);
        }
        for &sw_effect_key in item_keys.sw_effects.iter() {
            SolarSystem::util_add_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_key, reuse_eupdates);
        }
    }
    fn update_projections(&mut self) {
        let mut projection_updates = Vec::new();
        for (fit_key, u_fit) in self.u_data.fits.iter() {
            let ship_radius = self.u_data.get_ship_radius_by_fit_key(fit_key);
            for module_key in u_fit.iter_module_keys() {
                record_projection(&mut projection_updates, &self.u_data, module_key, ship_radius);
                let u_module = self.u_data.items.get(module_key).dc_module().unwrap();
                if let Some(charge_key) = u_module.get_charge_key() {
                    record_projection(&mut projection_updates, &self.u_data, charge_key, ship_radius);
                }
            }
            for &drone_key in u_fit.drones.iter() {
                let drone_radius = self.u_data.items.get(drone_key).get_direct_radius();
                record_projection(&mut projection_updates, &self.u_data, drone_key, drone_radius);
            }
            for &fighter_key in u_fit.fighters.iter() {
                let fighter_radius = self.u_data.items.get(fighter_key).get_direct_radius();
                record_projection(&mut projection_updates, &self.u_data, fighter_key, fighter_radius);
            }
        }
        for (projector_key, projectee_key, src_rad, tgt_rag) in projection_updates {
            let projector_u_item = self.u_data.items.get_mut(projector_key);
            projector_u_item
                .get_projs_mut()
                .unwrap()
                .get_proj_data_mut(&projectee_key)
                .unwrap()
                .update_radii(src_rad, tgt_rag);
        }
    }
}

fn record_projection(
    projection_updates: &mut Vec<(UItemKey, UItemKey, AttrVal, AttrVal)>,
    u_data: &UData,
    item_key: UItemKey,
    src_rad: AttrVal,
) {
    let u_item = u_data.items.get(item_key);
    for (projectee_key, _u_proj_data) in u_item.get_projs().unwrap().iter_projectees_and_datas() {
        let projectee_radius = u_data.items.get(projectee_key).get_direct_radius();
        projection_updates.push((item_key, projectee_key, src_rad, projectee_radius));
    }
}
