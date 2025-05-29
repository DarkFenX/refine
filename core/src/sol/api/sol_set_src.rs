use crate::{
    sol::{
        ItemKey, SolarSystem,
        uad::{
            Uad,
            item::{ShipKind, UadItem},
        },
    },
    src::Src,
};

struct ItemKeys {
    boosters: Vec<ItemKey>,
    characters: Vec<ItemKey>,
    charges: Vec<ItemKey>,
    drones: Vec<ItemKey>,
    fighters: Vec<ItemKey>,
    fw_effects: Vec<ItemKey>,
    implants: Vec<ItemKey>,
    modules: Vec<ItemKey>,
    proj_effects: Vec<ItemKey>,
    services: Vec<ItemKey>,
    rigs: Vec<ItemKey>,
    ships: Vec<ItemKey>,
    skills: Vec<ItemKey>,
    stances: Vec<ItemKey>,
    subsystems: Vec<ItemKey>,
    sw_effects: Vec<ItemKey>,
}
impl ItemKeys {
    fn from_uad(uad: &Uad) -> Self {
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
        for (item_key, uad_item) in uad.items.iter() {
            match uad_item {
                // Autocharges are added/removed by whichever item is carrying them (e.g. fighter)
                UadItem::Autocharge(_) => (),
                UadItem::Booster(_) => data.boosters.push(item_key),
                UadItem::Character(_) => data.characters.push(item_key),
                UadItem::Charge(_) => data.charges.push(item_key),
                UadItem::Drone(_) => data.drones.push(item_key),
                UadItem::Fighter(_) => data.fighters.push(item_key),
                UadItem::FwEffect(_) => data.fw_effects.push(item_key),
                UadItem::Implant(_) => data.implants.push(item_key),
                UadItem::Module(_) => data.modules.push(item_key),
                UadItem::ProjEffect(_) => data.proj_effects.push(item_key),
                UadItem::Service(_) => data.services.push(item_key),
                UadItem::Rig(_) => data.rigs.push(item_key),
                UadItem::Ship(_) => data.ships.push(item_key),
                UadItem::Skill(_) => data.skills.push(item_key),
                UadItem::Stance(_) => data.stances.push(item_key),
                UadItem::Subsystem(_) => data.subsystems.push(item_key),
                UadItem::SwEffect(_) => data.sw_effects.push(item_key),
            }
        }
        data
    }
}

impl SolarSystem {
    pub fn set_src(&mut self, mut src: Src) {
        let item_keys = ItemKeys::from_uad(&self.uad);
        self.unload_items(&item_keys);
        // Set new source, update source-dependent data in services and reload items
        std::mem::swap(&mut self.uad.src, &mut src);
        self.svc.notify_src_changed(&self.uad.src);
        for item in self.uad.items.values_mut() {
            item.update_a_data(&self.uad.src)
        }
        // Update fit kind
        for fit in self.uad.fits.values_mut() {
            fit.kind = match fit.ship {
                Some(ship_key) => self.uad.items.get(ship_key).get_ship().unwrap().get_kind(),
                None => ShipKind::Unknown,
            }
        }
        self.load_items(&item_keys);
    }
    fn unload_items(&mut self, item_keys: &ItemKeys) {
        for &booster_item_key in item_keys.boosters.iter() {
            let uad_item = self.uad.items.get(booster_item_key);
            SolarSystem::util_remove_booster(&self.uad, &mut self.svc, &mut self.reffs, booster_item_key, uad_item);
        }
        for &character_item_key in item_keys.characters.iter() {
            let uad_item = self.uad.items.get(character_item_key);
            SolarSystem::util_remove_character(&self.uad, &mut self.svc, &mut self.reffs, character_item_key, uad_item);
        }
        for &charge_item_key in item_keys.charges.iter() {
            let uad_item = self.uad.items.get(charge_item_key);
            SolarSystem::util_remove_charge_with_projs(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                charge_item_key,
                uad_item,
            );
        }
        for &drone_item_key in item_keys.drones.iter() {
            let uad_item = self.uad.items.get(drone_item_key);
            SolarSystem::util_remove_drone_with_projs(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                drone_item_key,
                uad_item,
            );
        }
        for &fighter_item_key in item_keys.fighters.iter() {
            SolarSystem::util_remove_fighter_with_projs(
                &mut self.uad,
                &mut self.svc,
                &mut self.reffs,
                &mut self.rprojs,
                fighter_item_key,
            );
        }
        for &fw_effect_item_key in item_keys.fw_effects.iter() {
            let uad_item = self.uad.items.get(fw_effect_item_key);
            SolarSystem::util_remove_fw_effect(&self.uad, &mut self.svc, &mut self.reffs, fw_effect_item_key, uad_item);
        }
        for &implant_item_key in item_keys.implants.iter() {
            let uad_item = self.uad.items.get(implant_item_key);
            SolarSystem::util_remove_implant(&self.uad, &mut self.svc, &mut self.reffs, implant_item_key, uad_item);
        }
        for &module_item_key in item_keys.modules.iter() {
            let uad_item = self.uad.items.get(module_item_key);
            SolarSystem::util_remove_module_with_projs(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                module_item_key,
                uad_item,
            );
        }
        for &proj_effect_item_key in item_keys.proj_effects.iter() {
            let uad_item = self.uad.items.get(proj_effect_item_key);
            SolarSystem::util_remove_proj_effect_with_projs(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                proj_effect_item_key,
                uad_item,
            );
        }
        for &service_item_key in item_keys.services.iter() {
            let uad_item = self.uad.items.get(service_item_key);
            SolarSystem::util_remove_service(&self.uad, &mut self.svc, &mut self.reffs, service_item_key, uad_item);
        }
        for &rig_item_key in item_keys.rigs.iter() {
            let uad_item = self.uad.items.get(rig_item_key);
            SolarSystem::util_remove_rig(&self.uad, &mut self.svc, &mut self.reffs, rig_item_key, uad_item);
        }
        for &ship_item_key in item_keys.ships.iter() {
            let uad_item = self.uad.items.get(ship_item_key);
            SolarSystem::util_remove_ship(&self.uad, &mut self.svc, &mut self.reffs, ship_item_key, uad_item);
        }
        for &skill_item_key in item_keys.skills.iter() {
            let uad_item = self.uad.items.get(skill_item_key);
            SolarSystem::util_remove_skill(&self.uad, &mut self.svc, &mut self.reffs, skill_item_key, uad_item);
        }
        for &stance_item_key in item_keys.stances.iter() {
            let uad_item = self.uad.items.get(stance_item_key);
            SolarSystem::util_remove_stance(&self.uad, &mut self.svc, &mut self.reffs, stance_item_key, uad_item);
        }
        for &subsystem_item_key in item_keys.subsystems.iter() {
            let uad_item = self.uad.items.get(subsystem_item_key);
            SolarSystem::util_remove_subsystem(&self.uad, &mut self.svc, &mut self.reffs, subsystem_item_key, uad_item);
        }
        for &sw_effect_item_key in item_keys.sw_effects.iter() {
            let uad_item = self.uad.items.get(sw_effect_item_key);
            SolarSystem::util_remove_sw_effect(&self.uad, &mut self.svc, &mut self.reffs, sw_effect_item_key, uad_item);
        }
    }
    fn load_items(&mut self, item_keys: &ItemKeys) {
        for &booster_item_key in item_keys.boosters.iter() {
            let uad_item = self.uad.items.get(booster_item_key);
            SolarSystem::util_add_booster(&self.uad, &mut self.svc, &mut self.reffs, booster_item_key, uad_item);
        }
        for &character_item_key in item_keys.characters.iter() {
            let uad_item = self.uad.items.get(character_item_key);
            SolarSystem::util_add_character(&self.uad, &mut self.svc, &mut self.reffs, character_item_key, uad_item);
        }
        for &charge_item_key in item_keys.charges.iter() {
            let uad_item = self.uad.items.get(charge_item_key);
            SolarSystem::util_add_charge_with_projs(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                charge_item_key,
                uad_item,
            );
        }
        for &drone_item_key in item_keys.drones.iter() {
            let uad_item = self.uad.items.get(drone_item_key);
            SolarSystem::util_add_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, drone_item_key, uad_item);
        }
        for &fighter_item_key in item_keys.fighters.iter() {
            SolarSystem::util_add_fighter_with_projs(
                &mut self.uad,
                &mut self.svc,
                &mut self.reffs,
                &mut self.rprojs,
                fighter_item_key,
            );
        }
        for &fw_effect_item_key in item_keys.fw_effects.iter() {
            let uad_item = self.uad.items.get(fw_effect_item_key);
            SolarSystem::util_add_fw_effect(&self.uad, &mut self.svc, &mut self.reffs, fw_effect_item_key, uad_item);
        }
        for &implant_item_key in item_keys.implants.iter() {
            let uad_item = self.uad.items.get(implant_item_key);
            SolarSystem::util_add_implant(&self.uad, &mut self.svc, &mut self.reffs, implant_item_key, uad_item);
        }
        for &module_item_key in item_keys.modules.iter() {
            let uad_item = self.uad.items.get(module_item_key);
            SolarSystem::util_add_module_with_projs(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                module_item_key,
                uad_item,
            );
        }
        for &proj_effect_item_key in item_keys.proj_effects.iter() {
            let uad_item = self.uad.items.get(proj_effect_item_key);
            SolarSystem::util_add_proj_effect_with_projs(
                &self.uad,
                &mut self.svc,
                &mut self.reffs,
                proj_effect_item_key,
                uad_item,
            );
        }
        for &service_item_key in item_keys.services.iter() {
            let uad_item = self.uad.items.get(service_item_key);
            SolarSystem::util_add_service(&self.uad, &mut self.svc, &mut self.reffs, service_item_key, uad_item);
        }
        for &rig_item_key in item_keys.rigs.iter() {
            let uad_item = self.uad.items.get(rig_item_key);
            SolarSystem::util_add_rig(&self.uad, &mut self.svc, &mut self.reffs, rig_item_key, uad_item);
        }
        for &ship_item_key in item_keys.ships.iter() {
            let uad_item = self.uad.items.get(ship_item_key);
            SolarSystem::util_add_ship(&self.uad, &mut self.svc, &mut self.reffs, ship_item_key, uad_item);
        }
        for &skill_item_key in item_keys.skills.iter() {
            let uad_item = self.uad.items.get(skill_item_key);
            SolarSystem::util_add_skill(&self.uad, &mut self.svc, &mut self.reffs, skill_item_key, uad_item);
        }
        for &stance_item_key in item_keys.stances.iter() {
            let uad_item = self.uad.items.get(stance_item_key);
            SolarSystem::util_add_stance(&self.uad, &mut self.svc, &mut self.reffs, stance_item_key, uad_item);
        }
        for &subsystem_item_key in item_keys.subsystems.iter() {
            let uad_item = self.uad.items.get(subsystem_item_key);
            SolarSystem::util_add_subsystem(&self.uad, &mut self.svc, &mut self.reffs, subsystem_item_key, uad_item);
        }
        for &sw_effect_item_key in item_keys.sw_effects.iter() {
            let uad_item = self.uad.items.get(sw_effect_item_key);
            SolarSystem::util_add_sw_effect(&self.uad, &mut self.svc, &mut self.reffs, sw_effect_item_key, uad_item);
        }
    }
}
