use itertools::chain;

use crate::{
    DpsProfile, FitId, FitSecStatus,
    ad::AItemId,
    ud::{
        UFleetId, UItemId,
        fit::{UFitSkill, UItemVec},
        item::UShipKind,
    },
    util::{LibGetId, LibNamed, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UFit {
    pub(crate) id: FitId,
    pub(crate) fleet: Option<UFleetId>,
    pub(crate) character: Option<UItemId>,
    pub(crate) skills: RMap<AItemId, UFitSkill>,
    pub(crate) implants: RSet<UItemId>,
    pub(crate) boosters: RSet<UItemId>,
    pub(crate) ship: Option<UItemId>,
    pub(crate) stance: Option<UItemId>,
    pub(crate) subsystems: RSet<UItemId>,
    pub(crate) mods_high: UItemVec,
    pub(crate) mods_mid: UItemVec,
    pub(crate) mods_low: UItemVec,
    pub(crate) rigs: RSet<UItemId>,
    pub(crate) services: RSet<UItemId>,
    pub(crate) drones: RSet<UItemId>,
    pub(crate) fighters: RSet<UItemId>,
    pub(crate) fw_effects: RSet<UItemId>,
    pub(crate) sec_status: FitSecStatus,
    pub(crate) rah_incoming_dps: Option<DpsProfile>,
    // Extra info for fast access
    pub(crate) ship_kind: UShipKind,
}
impl UFit {
    pub(crate) fn new(id: FitId) -> Self {
        Self {
            id,
            fleet: None,
            character: None,
            skills: RMap::new(),
            implants: RSet::new(),
            boosters: RSet::new(),
            ship: None,
            stance: None,
            subsystems: RSet::new(),
            mods_high: UItemVec::new(),
            mods_mid: UItemVec::new(),
            mods_low: UItemVec::new(),
            rigs: RSet::new(),
            services: RSet::new(),
            drones: RSet::new(),
            fighters: RSet::new(),
            fw_effects: RSet::new(),
            sec_status: FitSecStatus::default(),
            rah_incoming_dps: None,
            ship_kind: UShipKind::Unknown,
        }
    }
    pub(crate) fn iter_module_uids(&self) -> impl Iterator<Item = UItemId> {
        chain!(
            self.mods_high.iter_uids(),
            self.mods_mid.iter_uids(),
            self.mods_low.iter_uids(),
        )
    }
    pub(crate) fn iter_direct_items(&self) -> impl Iterator<Item = UItemId> {
        let Self {
            character,
            skills,
            implants,
            boosters,
            ship,
            stance,
            subsystems,
            mods_high,
            mods_mid,
            mods_low,
            rigs,
            services,
            drones,
            fighters,
            fw_effects,
            // Not items
            id: _,
            fleet: _,
            sec_status: _,
            rah_incoming_dps: _,
            ship_kind: _,
        } = self;
        chain!(
            *character,
            skills.values().map(|v| v.skill_uid),
            implants.iter().copied(),
            boosters.iter().copied(),
            *ship,
            *stance,
            subsystems.iter().copied(),
            mods_high.iter_uids(),
            mods_mid.iter_uids(),
            mods_low.iter_uids(),
            rigs.iter().copied(),
            services.iter().copied(),
            drones.iter().copied(),
            fighters.iter().copied(),
            fw_effects.iter().copied(),
        )
    }
}
impl LibNamed for UFit {
    fn lib_get_name() -> &'static str {
        "UFit"
    }
}
impl LibGetId<FitId> for UFit {
    fn lib_get_id(&self) -> FitId {
        self.id
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{
        ModRack,
        dbg::DebugResult,
        ud::{UData, UFighter, UFitId, UItem, UModule},
    };

    impl UFit {
        pub(in crate::ud) fn consistency_check(
            &self,
            u_data: &UData,
            seen_item_uids: &mut Vec<UItemId>,
        ) -> DebugResult {
            let Self {
                id,
                fleet,
                character,
                skills,
                implants,
                boosters,
                ship,
                stance,
                subsystems,
                mods_high,
                mods_mid,
                mods_low,
                rigs,
                services,
                drones,
                fighters,
                fw_effects,
                // Not checked
                sec_status: _,
                rah_incoming_dps: _,
                ship_kind: _,
            } = self;
            let Some(fit_uid) = u_data.fits.int_id_by_ext_id(id) else {
                return Err(Default::default());
            };
            // If fleet is defined, it should exist and refer fit back
            if let Some(fleet_uid) = *fleet {
                let Some(fleet) = u_data.fleets.try_get(fleet_uid) else {
                    return Err(Default::default());
                };
                if !fleet.contains_fit(&fit_uid) {
                    return Err(Default::default());
                }
            }
            // Character
            if let Some(character_uid) = *character {
                seen_item_uids.push(character_uid);
                let Some(item) = u_data.items.try_get(character_uid) else {
                    return Err(Default::default());
                };
                let UItem::Character(character) = item else {
                    return Err(Default::default());
                };
                if character.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Skills
            for fit_skill in skills.values() {
                seen_item_uids.push(fit_skill.skill_uid);
                let Some(item) = u_data.items.try_get(fit_skill.skill_uid) else {
                    return Err(Default::default());
                };
                let UItem::Skill(skill) = item else {
                    return Err(Default::default());
                };
                if skill.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                if skill.get_level() != fit_skill.level {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Implants
            for &implant_uid in implants.iter() {
                seen_item_uids.push(implant_uid);
                let Some(item) = u_data.items.try_get(implant_uid) else {
                    return Err(Default::default());
                };
                let UItem::Implant(implant) = item else {
                    return Err(Default::default());
                };
                if implant.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Boosters
            for &booster_uid in boosters.iter() {
                seen_item_uids.push(booster_uid);
                let Some(item) = u_data.items.try_get(booster_uid) else {
                    return Err(Default::default());
                };
                let UItem::Booster(booster) = item else {
                    return Err(Default::default());
                };
                if booster.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Ship
            if let Some(ship_uid) = *ship {
                seen_item_uids.push(ship_uid);
                let Some(item) = u_data.items.try_get(ship_uid) else {
                    return Err(Default::default());
                };
                let UItem::Ship(ship) = item else {
                    return Err(Default::default());
                };
                if ship.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Stance
            if let Some(stance_uid) = *stance {
                seen_item_uids.push(stance_uid);
                let Some(item) = u_data.items.try_get(stance_uid) else {
                    return Err(Default::default());
                };
                let UItem::Stance(stance) = item else {
                    return Err(Default::default());
                };
                if stance.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Subsystems
            for &subsystem_uid in subsystems.iter() {
                seen_item_uids.push(subsystem_uid);
                let Some(item) = u_data.items.try_get(subsystem_uid) else {
                    return Err(Default::default());
                };
                let UItem::Subsystem(subsystem) = item else {
                    return Err(Default::default());
                };
                if subsystem.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // High slot modules
            for module_uid in mods_high.iter_uids() {
                seen_item_uids.push(module_uid);
                let Some(item) = u_data.items.try_get(module_uid) else {
                    return Err(Default::default());
                };
                let UItem::Module(module) = item else {
                    return Err(Default::default());
                };
                if module.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                if !matches!(module.get_rack(), ModRack::High) {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
                check_module_charge(u_data, fit_uid, module_uid, module, seen_item_uids)?;
            }
            // Mid slot modules
            for module_uid in mods_mid.iter_uids() {
                seen_item_uids.push(module_uid);
                let Some(item) = u_data.items.try_get(module_uid) else {
                    return Err(Default::default());
                };
                let UItem::Module(module) = item else {
                    return Err(Default::default());
                };
                if module.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                if !matches!(module.get_rack(), ModRack::Mid) {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
                check_module_charge(u_data, fit_uid, module_uid, module, seen_item_uids)?;
            }
            // Low slot modules
            for module_uid in mods_low.iter_uids() {
                seen_item_uids.push(module_uid);
                let Some(item) = u_data.items.try_get(module_uid) else {
                    return Err(Default::default());
                };
                let UItem::Module(module) = item else {
                    return Err(Default::default());
                };
                if module.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                if !matches!(module.get_rack(), ModRack::Low) {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
                check_module_charge(u_data, fit_uid, module_uid, module, seen_item_uids)?;
            }
            // Rigs
            for &rig_uid in rigs.iter() {
                seen_item_uids.push(rig_uid);
                let Some(item) = u_data.items.try_get(rig_uid) else {
                    return Err(Default::default());
                };
                let UItem::Rig(rig) = item else {
                    return Err(Default::default());
                };
                if rig.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Services
            for &service_uid in services.iter() {
                seen_item_uids.push(service_uid);
                let Some(item) = u_data.items.try_get(service_uid) else {
                    return Err(Default::default());
                };
                let UItem::Service(service) = item else {
                    return Err(Default::default());
                };
                if service.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Drones
            for &drone_uid in drones.iter() {
                seen_item_uids.push(drone_uid);
                let Some(item) = u_data.items.try_get(drone_uid) else {
                    return Err(Default::default());
                };
                let UItem::Drone(drone) = item else {
                    return Err(Default::default());
                };
                if drone.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            // Fighters
            for &fighter_uid in fighters.iter() {
                seen_item_uids.push(fighter_uid);
                let Some(item) = u_data.items.try_get(fighter_uid) else {
                    return Err(Default::default());
                };
                let UItem::Fighter(fighter) = item else {
                    return Err(Default::default());
                };
                if fighter.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
                check_fighter_autocharges(u_data, fit_uid, fighter_uid, fighter, seen_item_uids)?;
            }
            // Fit-wide effects
            for &fw_effect_uid in fw_effects.iter() {
                seen_item_uids.push(fw_effect_uid);
                let Some(item) = u_data.items.try_get(fw_effect_uid) else {
                    return Err(Default::default());
                };
                let UItem::FwEffect(fw_effect) = item else {
                    return Err(Default::default());
                };
                if fw_effect.get_fit_uid() != fit_uid {
                    return Err(Default::default());
                }
                item.consistency_check(u_data)?;
            }
            Ok(())
        }
    }

    fn check_module_charge(
        u_data: &UData,
        fit_uid: UFitId,
        module_uid: UItemId,
        module: &UModule,
        seen_items: &mut Vec<UItemId>,
    ) -> DebugResult {
        if let Some(charge_uid) = module.get_charge_uid() {
            seen_items.push(charge_uid);
            let Some(item) = u_data.items.try_get(charge_uid) else {
                return Err(Default::default());
            };
            let UItem::Charge(charge) = item else {
                return Err(Default::default());
            };
            if charge.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            if charge.get_cont_item_uid() != module_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        Ok(())
    }

    fn check_fighter_autocharges(
        u_data: &UData,
        fit_uid: UFitId,
        fighter_uid: UItemId,
        fighter: &UFighter,
        seen_items: &mut Vec<UItemId>,
    ) -> DebugResult {
        for autocharge_uid in fighter.get_autocharges().values() {
            seen_items.push(autocharge_uid);
            let Some(item) = u_data.items.try_get(autocharge_uid) else {
                return Err(Default::default());
            };
            let UItem::Autocharge(autocharge) = item else {
                return Err(Default::default());
            };
            if autocharge.get_fit_uid() != fit_uid {
                return Err(Default::default());
            }
            if autocharge.get_cont_item_uid() != fighter_uid {
                return Err(Default::default());
            }
            item.consistency_check(u_data)?;
        }
        Ok(())
    }
}
