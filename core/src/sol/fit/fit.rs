use crate::{
    defs::{SolFitId, SolFleetId, SolItemId},
    sol::{
        item::{SolItem, SolShipKind},
        SolModRack,
    },
    util::StSet,
};

pub(in crate::sol) struct SolFit {
    pub(in crate::sol) id: SolFitId,
    pub(in crate::sol) kind: SolShipKind,
    pub(in crate::sol) fleet: Option<SolFleetId>,
    pub(in crate::sol) character: Option<SolItemId>,
    pub(in crate::sol) skills: StSet<SolItemId>,
    pub(in crate::sol) implants: StSet<SolItemId>,
    pub(in crate::sol) boosters: StSet<SolItemId>,
    pub(in crate::sol) ship: Option<SolItemId>,
    pub(in crate::sol) stance: Option<SolItemId>,
    pub(in crate::sol) subsystems: StSet<SolItemId>,
    pub(in crate::sol) mods_high: StSet<SolItemId>,
    pub(in crate::sol) mods_mid: StSet<SolItemId>,
    pub(in crate::sol) mods_low: StSet<SolItemId>,
    pub(in crate::sol) rigs: StSet<SolItemId>,
    pub(in crate::sol) drones: StSet<SolItemId>,
    pub(in crate::sol) fighters: StSet<SolItemId>,
    pub(in crate::sol) fw_effects: StSet<SolItemId>,
}
impl SolFit {
    pub(in crate::sol) fn new(id: SolFitId) -> Self {
        Self {
            id,
            kind: SolShipKind::default(),
            fleet: None,
            character: None,
            skills: StSet::new(),
            implants: StSet::new(),
            boosters: StSet::new(),
            ship: None,
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
    pub(in crate::sol) fn add_item(&mut self, item: &SolItem) {
        match item {
            SolItem::Character(character) => self.character = Some(character.base.id),
            SolItem::Skill(skill) => {
                self.skills.insert(skill.base.id);
                ()
            }
            SolItem::Implant(implant) => {
                self.implants.insert(implant.base.id);
                ()
            }
            SolItem::Booster(booster) => {
                self.boosters.insert(booster.base.id);
                ()
            }
            SolItem::Ship(ship) => {
                self.ship = Some(ship.base.id);
                self.kind = ship.kind;
            }
            SolItem::Stance(stance) => self.stance = Some(stance.base.id),
            SolItem::Subsystem(subsystem) => {
                self.subsystems.insert(subsystem.base.id);
                ()
            }
            SolItem::Module(module) => {
                match module.rack {
                    SolModRack::High => self.mods_high.insert(module.base.id),
                    SolModRack::Mid => self.mods_mid.insert(module.base.id),
                    SolModRack::Low => self.mods_low.insert(module.base.id),
                };
                ()
            }
            SolItem::Rig(rig) => {
                self.rigs.insert(rig.base.id);
                ()
            }
            SolItem::Drone(drone) => {
                self.drones.insert(drone.base.id);
                ()
            }
            SolItem::Fighter(fighter) => {
                self.fighters.insert(fighter.base.id);
                ()
            }
            SolItem::FwEffect(fw_effect) => {
                self.fw_effects.insert(fw_effect.base.id);
                ()
            }
            // Ignore charges and system-wide effects
            SolItem::Charge(_) => (),
            SolItem::AutoCharge(_) => (),
            SolItem::SwEffect(_) => (),
            SolItem::ProjEffect(_) => (),
        };
    }
    pub(in crate::sol) fn remove_item(&mut self, item: &SolItem) {
        match item {
            SolItem::Character(character) => {
                if self.character == Some(character.base.id) {
                    self.character = None
                }
            }
            SolItem::Skill(skill) => {
                self.skills.remove(&skill.base.id);
                ()
            }
            SolItem::Implant(implant) => {
                self.implants.remove(&implant.base.id);
                ()
            }
            SolItem::Booster(booster) => {
                self.boosters.remove(&booster.base.id);
                ()
            }
            SolItem::Ship(ship) => {
                if self.ship == Some(ship.base.id) {
                    self.ship = None;
                    self.kind = SolShipKind::default();
                }
            }
            SolItem::Stance(stance) => {
                if self.stance == Some(stance.base.id) {
                    self.stance = None
                }
            }
            SolItem::Subsystem(subsystem) => {
                self.subsystems.remove(&subsystem.base.id);
                ()
            }
            SolItem::Module(module) => {
                match module.rack {
                    SolModRack::High => self.mods_high.remove(&module.base.id),
                    SolModRack::Mid => self.mods_mid.remove(&module.base.id),
                    SolModRack::Low => self.mods_low.remove(&module.base.id),
                };
                ()
            }
            SolItem::Rig(rig) => {
                self.rigs.remove(&rig.base.id);
                ()
            }
            SolItem::Drone(drone) => {
                self.drones.remove(&drone.base.id);
            }
            SolItem::Fighter(fighter) => {
                self.fighters.remove(&fighter.base.id);
                ()
            }
            SolItem::FwEffect(fw_effect) => {
                self.fw_effects.remove(&fw_effect.base.id);
                ()
            }
            // Ignore charges and system-wide effects
            SolItem::Charge(_) => (),
            SolItem::AutoCharge(_) => (),
            SolItem::SwEffect(_) => (),
            SolItem::ProjEffect(_) => (),
        }
    }
    pub(in crate::sol) fn all_items(&self) -> Vec<SolItemId> {
        let mut items = Vec::new();
        conditional_push(&mut items, self.character);
        items.extend(self.skills.iter());
        items.extend(self.implants.iter());
        items.extend(self.boosters.iter());
        conditional_push(&mut items, self.ship);
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

fn conditional_push(items: &mut Vec<SolItemId>, opt_value: Option<SolItemId>) {
    if let Some(value) = opt_value {
        items.push(value)
    }
}
