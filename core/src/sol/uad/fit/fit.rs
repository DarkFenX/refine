use ordered_float::OrderedFloat as OF;

use crate::{
    sol::{
        FitId, FleetId, ItemId, ItemTypeId, SecStatus,
        misc::DpsProfile,
        uad::{
            fit::{FitSkill, ItemVec},
            item::ShipKind,
        },
    },
    util::{StMap, StSet},
};

#[derive(Clone)]
pub(in crate::sol) struct Fit {
    pub(in crate::sol) id: FitId,
    pub(in crate::sol) kind: ShipKind,
    pub(in crate::sol) fleet: Option<FleetId>,
    pub(in crate::sol) character: Option<ItemId>,
    pub(in crate::sol) skills: StMap<ItemTypeId, FitSkill>,
    pub(in crate::sol) implants: StSet<ItemId>,
    pub(in crate::sol) boosters: StSet<ItemId>,
    pub(in crate::sol) ship: Option<ItemId>,
    pub(in crate::sol) stance: Option<ItemId>,
    pub(in crate::sol) subsystems: StSet<ItemId>,
    pub(in crate::sol) mods_high: ItemVec,
    pub(in crate::sol) mods_mid: ItemVec,
    pub(in crate::sol) mods_low: ItemVec,
    pub(in crate::sol) rigs: StSet<ItemId>,
    pub(in crate::sol) services: StSet<ItemId>,
    pub(in crate::sol) drones: StSet<ItemId>,
    pub(in crate::sol) fighters: StSet<ItemId>,
    pub(in crate::sol) fw_effects: StSet<ItemId>,
    pub(in crate::sol) sec_status: SecStatus,
    pub(in crate::sol) rah_incoming_dps: Option<DpsProfile>,
}
impl Fit {
    pub(in crate::sol) fn new(id: FitId) -> Self {
        Self {
            id,
            kind: ShipKind::Unknown,
            fleet: None,
            character: None,
            skills: StMap::new(),
            implants: StSet::new(),
            boosters: StSet::new(),
            ship: None,
            stance: None,
            subsystems: StSet::new(),
            mods_high: ItemVec::new(),
            mods_mid: ItemVec::new(),
            mods_low: ItemVec::new(),
            rigs: StSet::new(),
            services: StSet::new(),
            drones: StSet::new(),
            fighters: StSet::new(),
            fw_effects: StSet::new(),
            sec_status: OF(0.0),
            rah_incoming_dps: None,
        }
    }
    pub(in crate::sol) fn all_direct_items(&self) -> Vec<ItemId> {
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

fn conditional_push(items: &mut Vec<ItemId>, opt_value: Option<ItemId>) {
    if let Some(value) = opt_value {
        items.push(value)
    }
}
