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
impl FitInfo {
    pub(in crate::sol) fn new(
        id: FitId,
        fleet: Option<FleetId>,
        character: Option<ItemId>,
        skills: Vec<ItemId>,
        implants: Vec<ItemId>,
        boosters: Vec<ItemId>,
        ship: Option<ItemId>,
        stance: Option<ItemId>,
        subsystems: Vec<ItemId>,
        mods_high: Vec<Option<ItemId>>,
        mods_mid: Vec<Option<ItemId>>,
        mods_low: Vec<Option<ItemId>>,
        rigs: Vec<ItemId>,
        drones: Vec<ItemId>,
        fighters: Vec<ItemId>,
        fw_effects: Vec<ItemId>,
        rah_incoming_dmg: Option<DmgProfile>,
    ) -> Self {
        Self {
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
            drones,
            fighters,
            fw_effects,
            rah_incoming_dmg,
        }
    }
}
impl From<&Fit> for FitInfo {
    fn from(fit: &Fit) -> Self {
        Self::new(
            fit.id,
            fit.fleet,
            fit.character,
            fit.skills.values().map(|v| v.item_id).collect(),
            fit.implants.iter().copied().collect(),
            fit.boosters.iter().copied().collect(),
            fit.ship,
            fit.stance,
            fit.subsystems.iter().copied().collect(),
            fit.mods_high.inner().clone(),
            fit.mods_mid.inner().clone(),
            fit.mods_low.inner().clone(),
            fit.rigs.iter().copied().collect(),
            fit.drones.iter().copied().collect(),
            fit.fighters.iter().copied().collect(),
            fit.fw_effects.iter().copied().collect(),
            fit.rah_incoming_dmg,
        )
    }
}
