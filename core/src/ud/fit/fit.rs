use itertools::chain;

use crate::{
    ad::AItemId,
    api::FitId,
    misc::{DpsProfile, FitSecStatus},
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
    pub(crate) fn iter_module_keys(&self) -> impl Iterator<Item = UItemId> {
        chain!(
            self.mods_high.iter_uids().copied(),
            self.mods_mid.iter_uids().copied(),
            self.mods_low.iter_uids().copied(),
        )
    }
    pub(crate) fn all_direct_items(&self) -> Vec<UItemId> {
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
        items.extend(self.skills.values().map(|v| v.skill_uid));
        items.extend(self.implants.iter());
        items.extend(self.boosters.iter());
        conditional_push(&mut items, self.ship);
        conditional_push(&mut items, self.stance);
        items.extend(self.subsystems.iter());
        items.extend(self.mods_high.iter_uids());
        items.extend(self.mods_mid.iter_uids());
        items.extend(self.mods_low.iter_uids());
        items.extend(self.rigs.iter());
        items.extend(self.services.iter());
        items.extend(self.drones.iter());
        items.extend(self.fighters.iter());
        items.extend(self.fw_effects.iter());
        items
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

fn conditional_push(items: &mut Vec<UItemId>, opt_value: Option<UItemId>) {
    if let Some(value) = opt_value {
        items.push(value)
    }
}
