use crate::sol::{
    DpsProfile, FitId, FleetId, ItemId, ItemKey, SecStatus,
    uad::{Uad, fit::Fit},
};

pub struct FitInfo {
    pub id: FitId,
    pub fleet: Option<FleetId>,
    pub character: Option<ItemId>,
    pub skills: Vec<ItemId>,
    pub implants: Vec<ItemId>,
    pub boosters: Vec<ItemId>,
    pub ship: Option<ItemId>,
    pub stance: Option<ItemId>,
    pub subsystems: Vec<ItemId>,
    pub mods_high: Vec<Option<ItemId>>,
    pub mods_mid: Vec<Option<ItemId>>,
    pub mods_low: Vec<Option<ItemId>>,
    pub rigs: Vec<ItemId>,
    pub services: Vec<ItemId>,
    pub drones: Vec<ItemId>,
    pub fighters: Vec<ItemId>,
    pub fw_effects: Vec<ItemId>,
    pub sec_status: SecStatus,
    pub rah_incoming_dps: Option<DpsProfile>,
}
impl FitInfo {
    pub(in crate::sol) fn from_fit(uad: &Uad, fit: &Fit) -> Self {
        Self {
            id: fit.id,
            fleet: fit.fleet.map(|fleet_key| uad.fleets.id_by_key(fleet_key)),
            character: conv_item_opt(uad, fit.character),
            skills: conv_item_iter(uad, fit.skills.values().map(|fit_skill| &fit_skill.item_key)),
            implants: conv_item_iter(uad, fit.implants.iter()),
            boosters: conv_item_iter(uad, fit.boosters.iter()),
            ship: conv_item_opt(uad, fit.ship),
            stance: conv_item_opt(uad, fit.stance),
            subsystems: conv_item_iter(uad, fit.subsystems.iter()),
            mods_high: conv_item_iter_opt(uad, fit.mods_high.iter_all()),
            mods_mid: conv_item_iter_opt(uad, fit.mods_mid.iter_all()),
            mods_low: conv_item_iter_opt(uad, fit.mods_low.iter_all()),
            rigs: conv_item_iter(uad, fit.rigs.iter()),
            services: conv_item_iter(uad, fit.services.iter()),
            drones: conv_item_iter(uad, fit.drones.iter()),
            fighters: conv_item_iter(uad, fit.fighters.iter()),
            fw_effects: conv_item_iter(uad, fit.fw_effects.iter()),
            sec_status: fit.sec_status,
            rah_incoming_dps: fit.rah_incoming_dps,
        }
    }
}

fn conv_item_opt(uad: &Uad, item_key: Option<ItemKey>) -> Option<ItemId> {
    item_key.map(|item_key| uad.items.id_by_key(item_key))
}

fn conv_item_iter<'a>(uad: &Uad, item_keys: impl Iterator<Item = &'a ItemKey>) -> Vec<ItemId> {
    item_keys.map(|item_key| uad.items.id_by_key(*item_key)).collect()
}

fn conv_item_iter_opt<'a>(uad: &Uad, item_keys: impl Iterator<Item = &'a Option<ItemKey>>) -> Vec<Option<ItemId>> {
    item_keys
        .map(|opt| opt.map(|item_key| uad.items.id_by_key(item_key)))
        .collect()
}
