use crate::sol::{DmgProfile, FitId, FleetId, ItemId, uad::fit::Fit};

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
    pub drones: Vec<ItemId>,
    pub fighters: Vec<ItemId>,
    pub fw_effects: Vec<ItemId>,
    pub rah_incoming_dmg: Option<DmgProfile>,
}
impl From<&Fit> for FitInfo {
    fn from(fit: &Fit) -> Self {
        Self {
            id: fit.id,
            fleet: fit.fleet,
            character: fit.character,
            skills: fit.skills.values().map(|v| v.item_id).collect(),
            implants: fit.implants.iter().copied().collect(),
            boosters: fit.boosters.iter().copied().collect(),
            ship: fit.ship,
            stance: fit.stance,
            subsystems: fit.subsystems.iter().copied().collect(),
            mods_high: fit.mods_high.inner().clone(),
            mods_mid: fit.mods_mid.inner().clone(),
            mods_low: fit.mods_low.inner().clone(),
            rigs: fit.rigs.iter().copied().collect(),
            drones: fit.drones.iter().copied().collect(),
            fighters: fit.fighters.iter().copied().collect(),
            fw_effects: fit.fw_effects.iter().copied().collect(),
            rah_incoming_dmg: fit.rah_incoming_dmg,
        }
    }
}
