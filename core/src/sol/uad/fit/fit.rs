use crate::{
    defs::{EItemId, SolFitId, SolFleetId, SolItemId},
    sol::{
        misc::SolDmgProfile,
        uad::{
            fit::{SolFitSkill, SolItemVec},
            item::SolShipKind,
        },
    },
    util::{StMap, StSet},
};

#[derive(Clone)]
pub(in crate::sol) struct SolFit {
    pub(in crate::sol) id: SolFitId,
    pub(in crate::sol) kind: SolShipKind,
    pub(in crate::sol) fleet: Option<SolFleetId>,
    pub(in crate::sol) character: Option<SolItemId>,
    pub(in crate::sol) skills: StMap<EItemId, SolFitSkill>,
    pub(in crate::sol) implants: StSet<SolItemId>,
    pub(in crate::sol) boosters: StSet<SolItemId>,
    pub(in crate::sol) ship: Option<SolItemId>,
    pub(in crate::sol) stance: Option<SolItemId>,
    pub(in crate::sol) subsystems: StSet<SolItemId>,
    pub(in crate::sol) mods_high: SolItemVec,
    pub(in crate::sol) mods_mid: SolItemVec,
    pub(in crate::sol) mods_low: SolItemVec,
    pub(in crate::sol) rigs: StSet<SolItemId>,
    pub(in crate::sol) services: StSet<SolItemId>,
    pub(in crate::sol) drones: StSet<SolItemId>,
    pub(in crate::sol) fighters: StSet<SolItemId>,
    pub(in crate::sol) fw_effects: StSet<SolItemId>,
    pub(in crate::sol) rah_incoming_dmg: Option<SolDmgProfile>,
}
impl SolFit {
    pub(in crate::sol) fn new(id: SolFitId) -> Self {
        Self {
            id,
            kind: SolShipKind::Unknown,
            fleet: None,
            character: None,
            skills: StMap::new(),
            implants: StSet::new(),
            boosters: StSet::new(),
            ship: None,
            stance: None,
            subsystems: StSet::new(),
            mods_high: SolItemVec::new(),
            mods_mid: SolItemVec::new(),
            mods_low: SolItemVec::new(),
            rigs: StSet::new(),
            services: StSet::new(),
            drones: StSet::new(),
            fighters: StSet::new(),
            fw_effects: StSet::new(),
            rah_incoming_dmg: None,
        }
    }
    pub(in crate::sol) fn all_direct_items(&self) -> Vec<SolItemId> {
        // Calculate capacity
        let mut capacity = 0;
        if self.character.is_some() {
            capacity += 1;
        }
        capacity += self.skills.len();
        capacity += self.implants.len();
        capacity += self.boosters.len();
        if self.ship.is_some() {
            capacity += 1;
        }
        if self.stance.is_some() {
            capacity += 1;
        }
        capacity += self.subsystems.len();
        capacity += self.mods_high.item_count();
        capacity += self.mods_mid.item_count();
        capacity += self.mods_low.item_count();
        capacity += self.rigs.len();
        capacity += self.services.len();
        capacity += self.drones.len();
        capacity += self.fighters.len();
        capacity += self.fw_effects.len();
        // Fill the data
        let mut items = Vec::with_capacity(capacity);
        conditional_push(&mut items, self.character);
        items.extend(self.skills.values().map(|v| v.item_id));
        items.extend(self.implants.iter());
        items.extend(self.boosters.iter());
        conditional_push(&mut items, self.ship);
        conditional_push(&mut items, self.stance);
        items.extend(self.subsystems.iter());
        items.extend(self.mods_high.iter_ids());
        items.extend(self.mods_mid.iter_ids());
        items.extend(self.mods_low.iter_ids());
        items.extend(self.rigs.iter());
        items.extend(self.services.iter());
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
