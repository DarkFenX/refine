use crate::{
    def::AttrVal,
    sol::SolarSystem,
    src::Src,
    ud::{UData, UEffectUpdates, UItem, UItemKey, UShipKind},
    util::RMap,
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
        let mut reuse_unload_eupdates = UEffectUpdates::new();
        self.unload_items(&item_keys, &mut reuse_unload_eupdates);
        // Set new source, update source-dependent data in services and reload items
        std::mem::swap(&mut self.u_data.src, &mut src);
        self.svc.notify_src_changed(&self.u_data.src);
        let mut load_eupdates_map = RMap::new();
        for (item_key, item) in self.u_data.items.iter_mut() {
            let mut item_eupdates = UEffectUpdates::new();
            item.update_a_data(&mut item_eupdates, &self.u_data.src);
            load_eupdates_map.insert(item_key, item_eupdates);
        }
        // Update fit kind
        for fit in self.u_data.fits.values_mut() {
            fit.kind = match fit.ship {
                Some(ship_key) => self.u_data.items.get(ship_key).get_ship().unwrap().get_kind(),
                None => UShipKind::Unknown,
            }
        }
        // Update on-projection data due to changed item radii
        self.update_projections();
        self.load_items(&item_keys, load_eupdates_map);
    }
    fn unload_items(&mut self, item_keys: &ItemKeys, reuse_eupdates: &mut UEffectUpdates) {
        for &booster_key in item_keys.boosters.iter() {
            let u_item = self.u_data.items.get(booster_key);
            SolarSystem::util_remove_booster(&self.u_data, &mut self.svc, booster_key, u_item, reuse_eupdates);
        }
        for &character_key in item_keys.characters.iter() {
            let u_item = self.u_data.items.get(character_key);
            SolarSystem::util_remove_character(&self.u_data, &mut self.svc, character_key, u_item, reuse_eupdates);
        }
        for &charge_key in item_keys.charges.iter() {
            let u_item = self.u_data.items.get(charge_key);
            SolarSystem::util_remove_charge_with_projs(&self.u_data, &mut self.svc, charge_key, u_item, reuse_eupdates);
        }
        for &drone_key in item_keys.drones.iter() {
            let u_item = self.u_data.items.get(drone_key);
            SolarSystem::util_remove_drone_with_projs(&self.u_data, &mut self.svc, drone_key, u_item, reuse_eupdates);
        }
        for &fighter_key in item_keys.fighters.iter() {
            SolarSystem::util_remove_fighter_with_projs(
                &mut self.u_data,
                &mut self.svc,
                &mut self.rev_projs,
                fighter_key,
                reuse_eupdates,
            );
        }
        for &fw_effect_key in item_keys.fw_effects.iter() {
            let u_item = self.u_data.items.get(fw_effect_key);
            SolarSystem::util_remove_fw_effect(&self.u_data, &mut self.svc, fw_effect_key, u_item, reuse_eupdates);
        }
        for &implant_key in item_keys.implants.iter() {
            let u_item = self.u_data.items.get(implant_key);
            SolarSystem::util_remove_implant(&self.u_data, &mut self.svc, implant_key, u_item, reuse_eupdates);
        }
        for &module_key in item_keys.modules.iter() {
            let u_item = self.u_data.items.get(module_key);
            SolarSystem::util_remove_module_with_projs(&self.u_data, &mut self.svc, module_key, u_item, reuse_eupdates);
        }
        for &proj_effect_key in item_keys.proj_effects.iter() {
            let u_item = self.u_data.items.get(proj_effect_key);
            SolarSystem::util_remove_proj_effect_with_projs(
                &self.u_data,
                &mut self.svc,
                proj_effect_key,
                u_item,
                reuse_eupdates,
            );
        }
        for &service_key in item_keys.services.iter() {
            let u_item = self.u_data.items.get(service_key);
            SolarSystem::util_remove_service(&self.u_data, &mut self.svc, service_key, u_item, reuse_eupdates);
        }
        for &rig_key in item_keys.rigs.iter() {
            let u_item = self.u_data.items.get(rig_key);
            SolarSystem::util_remove_rig(&self.u_data, &mut self.svc, rig_key, u_item, reuse_eupdates);
        }
        for &ship_key in item_keys.ships.iter() {
            let u_item = self.u_data.items.get(ship_key);
            SolarSystem::util_remove_ship(&self.u_data, &mut self.svc, ship_key, u_item, reuse_eupdates);
        }
        for &skill_key in item_keys.skills.iter() {
            let u_item = self.u_data.items.get(skill_key);
            SolarSystem::util_remove_skill(&self.u_data, &mut self.svc, skill_key, u_item, reuse_eupdates);
        }
        for &stance_key in item_keys.stances.iter() {
            let u_item = self.u_data.items.get(stance_key);
            SolarSystem::util_remove_stance(&self.u_data, &mut self.svc, stance_key, u_item, reuse_eupdates);
        }
        for &subsystem_key in item_keys.subsystems.iter() {
            let u_item = self.u_data.items.get(subsystem_key);
            SolarSystem::util_remove_subsystem(&self.u_data, &mut self.svc, subsystem_key, u_item, reuse_eupdates);
        }
        for &sw_effect_key in item_keys.sw_effects.iter() {
            let u_item = self.u_data.items.get(sw_effect_key);
            SolarSystem::util_remove_sw_effect(&self.u_data, &mut self.svc, sw_effect_key, u_item, reuse_eupdates);
        }
    }
    fn load_items(&mut self, item_keys: &ItemKeys, eupdates_map: RMap<UItemKey, UEffectUpdates>) {
        for &booster_key in item_keys.boosters.iter() {
            let booster_eupdates = eupdates_map.get(&booster_key).unwrap();
            SolarSystem::util_add_booster(&self.u_data, &mut self.svc, booster_key, booster_eupdates);
        }
        for &character_key in item_keys.characters.iter() {
            let character_eupdates = eupdates_map.get(&character_key).unwrap();
            SolarSystem::util_add_character(&self.u_data, &mut self.svc, character_key, character_eupdates);
        }
        for &charge_key in item_keys.charges.iter() {
            let charge_eupdates = eupdates_map.get(&charge_key).unwrap();
            SolarSystem::util_add_charge_with_projs(&self.u_data, &mut self.svc, charge_key, charge_eupdates);
        }
        for &drone_key in item_keys.drones.iter() {
            let drone_eupdates = eupdates_map.get(&drone_key).unwrap();
            SolarSystem::util_add_drone_with_projs(&self.u_data, &mut self.svc, drone_key, drone_eupdates);
        }
        for &fighter_key in item_keys.fighters.iter() {
            let fighter_eupdates = eupdates_map.get(&fighter_key).unwrap();
            SolarSystem::util_add_fighter_with_projs(
                &mut self.u_data,
                &mut self.svc,
                &mut self.rev_projs,
                fighter_key,
                fighter_eupdates,
            );
        }
        for &fw_effect_key in item_keys.fw_effects.iter() {
            let fw_effect_eupdates = eupdates_map.get(&fw_effect_key).unwrap();
            SolarSystem::util_add_fw_effect(&self.u_data, &mut self.svc, fw_effect_key, fw_effect_eupdates);
        }
        for &implant_key in item_keys.implants.iter() {
            let implant_eupdates = eupdates_map.get(&implant_key).unwrap();
            SolarSystem::util_add_implant(&self.u_data, &mut self.svc, implant_key, implant_eupdates);
        }
        for &module_key in item_keys.modules.iter() {
            let module_eupdates = eupdates_map.get(&module_key).unwrap();
            SolarSystem::util_add_module_with_projs(&self.u_data, &mut self.svc, module_key, module_eupdates);
        }
        for &proj_effect_key in item_keys.proj_effects.iter() {
            let proj_effect_eupdates = eupdates_map.get(&proj_effect_key).unwrap();
            SolarSystem::util_add_proj_effect_with_projs(
                &self.u_data,
                &mut self.svc,
                proj_effect_key,
                proj_effect_eupdates,
            );
        }
        for &service_key in item_keys.services.iter() {
            let service_eupdates = eupdates_map.get(&service_key).unwrap();
            SolarSystem::util_add_service(&self.u_data, &mut self.svc, service_key, service_eupdates);
        }
        for &rig_key in item_keys.rigs.iter() {
            let rig_eupdates = eupdates_map.get(&rig_key).unwrap();
            SolarSystem::util_add_rig(&self.u_data, &mut self.svc, rig_key, rig_eupdates);
        }
        for &ship_key in item_keys.ships.iter() {
            let ship_eupdates = eupdates_map.get(&ship_key).unwrap();
            SolarSystem::util_add_ship(&self.u_data, &mut self.svc, ship_key, ship_eupdates);
        }
        for &skill_key in item_keys.skills.iter() {
            let skill_eupdates = eupdates_map.get(&skill_key).unwrap();
            SolarSystem::util_add_skill(&self.u_data, &mut self.svc, skill_key, skill_eupdates);
        }
        for &stance_key in item_keys.stances.iter() {
            let stance_eupdates = eupdates_map.get(&stance_key).unwrap();
            SolarSystem::util_add_stance(&self.u_data, &mut self.svc, stance_key, stance_eupdates);
        }
        for &subsystem_key in item_keys.subsystems.iter() {
            let subsystem_eupdates = eupdates_map.get(&subsystem_key).unwrap();
            SolarSystem::util_add_subsystem(&self.u_data, &mut self.svc, subsystem_key, subsystem_eupdates);
        }
        for &sw_effect_key in item_keys.sw_effects.iter() {
            let sw_effect_eupdates = eupdates_map.get(&sw_effect_key).unwrap();
            SolarSystem::util_add_sw_effect(&self.u_data, &mut self.svc, sw_effect_key, sw_effect_eupdates);
        }
    }
    fn update_projections(&mut self) {
        let mut projection_updates = Vec::new();
        for (fit_key, u_fit) in self.u_data.fits.iter() {
            let ship_radius = self.u_data.get_ship_radius_by_fit_key(fit_key);
            for module_key in u_fit.iter_module_keys() {
                record_projection(&mut projection_updates, &self.u_data, module_key, ship_radius);
                let u_module = self.u_data.items.get(module_key).get_module().unwrap();
                if let Some(charge_key) = u_module.get_charge_key() {
                    record_projection(&mut projection_updates, &self.u_data, charge_key, ship_radius);
                }
            }
            for &drone_key in u_fit.drones.iter() {
                let drone_radius = self.u_data.get_item_radius(drone_key);
                record_projection(&mut projection_updates, &self.u_data, drone_key, drone_radius);
            }
            for &fighter_key in u_fit.fighters.iter() {
                let fighter_radius = self.u_data.get_item_radius(fighter_key);
                record_projection(&mut projection_updates, &self.u_data, fighter_key, fighter_radius);
            }
        }
        for (projector_key, projectee_key, src_rad, tgt_rag) in projection_updates {
            let projector_u_item = self.u_data.items.get_mut(projector_key);
            projector_u_item
                .get_projs_mut()
                .unwrap()
                .get_range_mut(&projectee_key)
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
    for (projectee_key, _u_prange) in u_item.get_projs().unwrap().iter_projectees_and_ranges() {
        let projectee_rad = u_data.get_item_radius(projectee_key);
        projection_updates.push((item_key, projectee_key, src_rad, projectee_rad));
    }
}
