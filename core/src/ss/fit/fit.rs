use crate::{
    defs::{SsFitId, SsFleetId, SsItemId},
    ss::{item::SsItem, SsModRack},
    util::StSet,
};

pub(in crate::ss) struct SsFit {
    pub(in crate::ss) id: SsFitId,
    pub(in crate::ss) fleet: Option<SsFleetId>,
    pub(in crate::ss) character: Option<SsItemId>,
    pub(in crate::ss) skills: StSet<SsItemId>,
    pub(in crate::ss) implants: StSet<SsItemId>,
    pub(in crate::ss) boosters: StSet<SsItemId>,
    pub(in crate::ss) ship: Option<SsItemId>,
    pub(in crate::ss) structure: Option<SsItemId>,
    pub(in crate::ss) stance: Option<SsItemId>,
    pub(in crate::ss) subsystems: StSet<SsItemId>,
    pub(in crate::ss) mods_high: StSet<SsItemId>,
    pub(in crate::ss) mods_mid: StSet<SsItemId>,
    pub(in crate::ss) mods_low: StSet<SsItemId>,
    pub(in crate::ss) rigs: StSet<SsItemId>,
    pub(in crate::ss) drones: StSet<SsItemId>,
    pub(in crate::ss) fighters: StSet<SsItemId>,
    pub(in crate::ss) fw_effects: StSet<SsItemId>,
}
impl SsFit {
    pub(in crate::ss) fn new(id: SsFitId) -> Self {
        Self {
            id,
            fleet: None,
            character: None,
            skills: StSet::new(),
            implants: StSet::new(),
            boosters: StSet::new(),
            ship: None,
            structure: None,
            stance: None,
            subsystems: StSet::new(),
            mods_high: StSet::new(),
            mods_mid: StSet::new(),
            mods_low: StSet::new(),
            rigs: StSet::new(),
            drones: StSet::new(),
            fighters: StSet::new(),
            fw_effects: StSet::new(),
        }
    }
    pub(in crate::ss) fn add_item(&mut self, item: &SsItem) {
        match item {
            SsItem::Character(character) => self.character = Some(character.id),
            SsItem::Skill(skill) => {
                self.skills.insert(skill.id);
                ()
            }
            SsItem::Implant(implant) => {
                self.implants.insert(implant.id);
                ()
            }
            SsItem::Booster(booster) => {
                self.boosters.insert(booster.id);
                ()
            }
            SsItem::Ship(ship) => self.ship = Some(ship.id),
            SsItem::Structure(structure) => self.structure = Some(structure.id),
            SsItem::Stance(stance) => self.stance = Some(stance.id),
            SsItem::Subsystem(subsystem) => {
                self.subsystems.insert(subsystem.id);
                ()
            }
            SsItem::Module(module) => {
                match module.rack {
                    SsModRack::High => self.mods_high.insert(module.id),
                    SsModRack::Mid => self.mods_mid.insert(module.id),
                    SsModRack::Low => self.mods_low.insert(module.id),
                };
                ()
            }
            SsItem::Rig(rig) => {
                self.rigs.insert(rig.id);
                ()
            }
            SsItem::Drone(drone) => {
                self.drones.insert(drone.id);
                ()
            }
            SsItem::Fighter(fighter) => {
                self.fighters.insert(fighter.id);
                ()
            }
            SsItem::FwEffect(fw_effect) => {
                self.fw_effects.insert(fw_effect.id);
                ()
            }
            // Ignore charges and system-wide effects
            SsItem::Charge(_) => (),
            SsItem::SwEffect(_) => (),
            SsItem::ProjEffect(_) => (),
        };
    }
    pub(in crate::ss) fn remove_item(&mut self, item: &SsItem) {
        match item {
            SsItem::Character(character) => {
                if self.character == Some(character.id) {
                    self.character = None
                }
            }
            SsItem::Skill(skill) => {
                self.skills.remove(&skill.id);
                ()
            }
            SsItem::Implant(implant) => {
                self.implants.remove(&implant.id);
                ()
            }
            SsItem::Booster(booster) => {
                self.boosters.remove(&booster.id);
                ()
            }
            SsItem::Ship(ship) => {
                if self.ship == Some(ship.id) {
                    self.ship = None
                }
            }
            SsItem::Structure(structure) => {
                if self.structure == Some(structure.id) {
                    self.structure = None
                }
            }
            SsItem::Stance(stance) => {
                if self.stance == Some(stance.id) {
                    self.stance = None
                }
            }
            SsItem::Subsystem(subsystem) => {
                self.subsystems.remove(&subsystem.id);
                ()
            }
            SsItem::Module(module) => {
                match module.rack {
                    SsModRack::High => self.mods_high.remove(&module.id),
                    SsModRack::Mid => self.mods_mid.remove(&module.id),
                    SsModRack::Low => self.mods_low.remove(&module.id),
                };
                ()
            }
            SsItem::Rig(rig) => {
                self.rigs.remove(&rig.id);
                ()
            }
            SsItem::Drone(drone) => {
                self.drones.remove(&drone.id);
            }
            SsItem::Fighter(fighter) => {
                self.fighters.remove(&fighter.id);
                ()
            }
            SsItem::FwEffect(fw_effect) => {
                self.fw_effects.remove(&fw_effect.id);
                ()
            }
            // Ignore charges and system-wide effects
            SsItem::Charge(_) => (),
            SsItem::SwEffect(_) => (),
            SsItem::ProjEffect(_) => (),
        }
    }
    pub(in crate::ss) fn all_items(&self) -> Vec<SsItemId> {
        let mut items = Vec::new();
        conditional_push(&mut items, self.character);
        items.extend(self.skills.iter());
        items.extend(self.implants.iter());
        items.extend(self.boosters.iter());
        conditional_push(&mut items, self.ship);
        conditional_push(&mut items, self.structure);
        conditional_push(&mut items, self.stance);
        items.extend(self.subsystems.iter());
        items.extend(self.mods_high.iter());
        items.extend(self.mods_mid.iter());
        items.extend(self.mods_low.iter());
        items.extend(self.rigs.iter());
        items.extend(self.drones.iter());
        items.extend(self.fighters.iter());
        items.extend(self.fw_effects.iter());
        items
    }
}

fn conditional_push(items: &mut Vec<SsItemId>, opt_value: Option<SsItemId>) {
    if let Some(value) = opt_value {
        items.push(value)
    }
}
