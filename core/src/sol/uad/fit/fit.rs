use crate::{
    sol::{
        FitId, FitSecStatus, FleetKey, ItemKey, ItemTypeId,
        misc::DpsProfile,
        uad::{
            fit::{FitSkill, ItemVec},
            item::ShipKind,
        },
    },
    util::{GetId, Named, RMap, RSet},
};

#[derive(Clone)]
pub(in crate::sol) struct UadFit {
    pub(in crate::sol) id: FitId,
    pub(in crate::sol) kind: ShipKind,
    pub(in crate::sol) fleet: Option<FleetKey>,
    pub(in crate::sol) character: Option<ItemKey>,
    pub(in crate::sol) skills: RMap<ItemTypeId, FitSkill>,
    pub(in crate::sol) implants: RSet<ItemKey>,
    pub(in crate::sol) boosters: RSet<ItemKey>,
    pub(in crate::sol) ship: Option<ItemKey>,
    pub(in crate::sol) stance: Option<ItemKey>,
    pub(in crate::sol) subsystems: RSet<ItemKey>,
    pub(in crate::sol) mods_high: ItemVec,
    pub(in crate::sol) mods_mid: ItemVec,
    pub(in crate::sol) mods_low: ItemVec,
    pub(in crate::sol) rigs: RSet<ItemKey>,
    pub(in crate::sol) services: RSet<ItemKey>,
    pub(in crate::sol) drones: RSet<ItemKey>,
    pub(in crate::sol) fighters: RSet<ItemKey>,
    pub(in crate::sol) fw_effects: RSet<ItemKey>,
    pub(in crate::sol) sec_status: FitSecStatus,
    pub(in crate::sol) rah_incoming_dps: Option<DpsProfile>,
}
impl UadFit {
    pub(in crate::sol) fn new(id: FitId) -> Self {
        Self {
            id,
            kind: ShipKind::Unknown,
            fleet: None,
            character: None,
            skills: RMap::new(),
            implants: RSet::new(),
            boosters: RSet::new(),
            ship: None,
            stance: None,
            subsystems: RSet::new(),
            mods_high: ItemVec::new(),
            mods_mid: ItemVec::new(),
            mods_low: ItemVec::new(),
            rigs: RSet::new(),
            services: RSet::new(),
            drones: RSet::new(),
            fighters: RSet::new(),
            fw_effects: RSet::new(),
            sec_status: FitSecStatus::new_clamped(0.0),
            rah_incoming_dps: None,
        }
    }
    pub(in crate::sol) fn all_direct_items(&self) -> Vec<ItemKey> {
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
        items.extend(self.skills.values().map(|v| v.item_key));
        items.extend(self.implants.iter());
        items.extend(self.boosters.iter());
        conditional_push(&mut items, self.ship);
        conditional_push(&mut items, self.stance);
        items.extend(self.subsystems.iter());
        items.extend(self.mods_high.iter_keys());
        items.extend(self.mods_mid.iter_keys());
        items.extend(self.mods_low.iter_keys());
        items.extend(self.rigs.iter());
        items.extend(self.services.iter());
        items.extend(self.drones.iter());
        items.extend(self.fighters.iter());
        items.extend(self.fw_effects.iter());
        items
    }
}
impl Named for UadFit {
    fn get_name() -> &'static str {
        "Fit"
    }
}
impl GetId<FitId> for UadFit {
    fn get_id(&self) -> FitId {
        self.id
    }
}

fn conditional_push(items: &mut Vec<ItemKey>, opt_value: Option<ItemKey>) {
    if let Some(value) = opt_value {
        items.push(value)
    }
}
