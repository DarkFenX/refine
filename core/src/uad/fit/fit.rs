use itertools::chain;

use crate::{
    ad,
    def::FitId,
    misc::{DpsProfile, FitSecStatus},
    uad::{
        UadFleetKey, UadItemKey,
        fit::{UadFitSkill, UadItemVec},
        item::ShipKind,
    },
    util::{GetId, Named, RMap, RSet},
};

#[derive(Clone)]
pub(crate) struct UadFit {
    pub(crate) id: FitId,
    pub(crate) kind: ShipKind,
    pub(crate) fleet: Option<UadFleetKey>,
    pub(crate) character: Option<UadItemKey>,
    pub(crate) skills: RMap<ad::AItemId, UadFitSkill>,
    pub(crate) implants: RSet<UadItemKey>,
    pub(crate) boosters: RSet<UadItemKey>,
    pub(crate) ship: Option<UadItemKey>,
    pub(crate) stance: Option<UadItemKey>,
    pub(crate) subsystems: RSet<UadItemKey>,
    pub(crate) mods_high: UadItemVec,
    pub(crate) mods_mid: UadItemVec,
    pub(crate) mods_low: UadItemVec,
    pub(crate) rigs: RSet<UadItemKey>,
    pub(crate) services: RSet<UadItemKey>,
    pub(crate) drones: RSet<UadItemKey>,
    pub(crate) fighters: RSet<UadItemKey>,
    pub(crate) fw_effects: RSet<UadItemKey>,
    pub(crate) sec_status: FitSecStatus,
    pub(crate) rah_incoming_dps: Option<DpsProfile>,
}
impl UadFit {
    pub(crate) fn new(id: FitId) -> Self {
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
            mods_high: UadItemVec::new(),
            mods_mid: UadItemVec::new(),
            mods_low: UadItemVec::new(),
            rigs: RSet::new(),
            services: RSet::new(),
            drones: RSet::new(),
            fighters: RSet::new(),
            fw_effects: RSet::new(),
            sec_status: FitSecStatus::new_clamped(0.0),
            rah_incoming_dps: None,
        }
    }
    pub(crate) fn iter_module_keys(&self) -> impl Iterator<Item = UadItemKey> {
        chain!(
            self.mods_high.iter_keys().copied(),
            self.mods_mid.iter_keys().copied(),
            self.mods_low.iter_keys().copied(),
        )
    }
    pub(crate) fn all_direct_items(&self) -> Vec<UadItemKey> {
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

fn conditional_push(items: &mut Vec<UadItemKey>, opt_value: Option<UadItemKey>) {
    if let Some(value) = opt_value {
        items.push(value)
    }
}
