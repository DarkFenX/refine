use crate::{
    defs::{SsFitId, SsFleetId, SsItemId},
    ss::fit::SsFit,
};

pub struct SsFitInfo {
    pub id: SsFitId,
    pub fleet: Option<SsFleetId>,
    pub character: Option<SsItemId>,
    pub skills: Vec<SsItemId>,
    pub implants: Vec<SsItemId>,
    pub boosters: Vec<SsItemId>,
    pub ship: Option<SsItemId>,
    pub structure: Option<SsItemId>,
    pub stance: Option<SsItemId>,
    pub subsystems: Vec<SsItemId>,
    pub mods_high: Vec<SsItemId>,
    pub mods_mid: Vec<SsItemId>,
    pub mods_low: Vec<SsItemId>,
    pub rigs: Vec<SsItemId>,
    pub drones: Vec<SsItemId>,
    pub fighters: Vec<SsItemId>,
    pub fw_effects: Vec<SsItemId>,
}
impl SsFitInfo {
    pub(in crate::ss) fn new(
        id: SsFitId,
        fleet: Option<SsFleetId>,
        character: Option<SsItemId>,
        skills: Vec<SsItemId>,
        implants: Vec<SsItemId>,
        boosters: Vec<SsItemId>,
        ship: Option<SsItemId>,
        structure: Option<SsItemId>,
        stance: Option<SsItemId>,
        subsystems: Vec<SsItemId>,
        mods_high: Vec<SsItemId>,
        mods_mid: Vec<SsItemId>,
        mods_low: Vec<SsItemId>,
        rigs: Vec<SsItemId>,
        drones: Vec<SsItemId>,
        fighters: Vec<SsItemId>,
        fw_effects: Vec<SsItemId>,
    ) -> Self {
        Self {
            id,
            fleet,
            character,
            skills,
            implants,
            boosters,
            ship,
            structure,
            stance,
            subsystems,
            mods_high,
            mods_mid,
            mods_low,
            rigs,
            drones,
            fighters,
            fw_effects,
        }
    }
}
impl From<&SsFit> for SsFitInfo {
    fn from(fit: &SsFit) -> Self {
        Self::new(
            fit.id,
            fit.fleet,
            fit.character,
            fit.skills.iter().map(|v| *v).collect(),
            fit.implants.iter().map(|v| *v).collect(),
            fit.boosters.iter().map(|v| *v).collect(),
            fit.ship,
            fit.structure,
            fit.stance,
            fit.subsystems.iter().map(|v| *v).collect(),
            fit.mods_high.iter().map(|v| *v).collect(),
            fit.mods_mid.iter().map(|v| *v).collect(),
            fit.mods_low.iter().map(|v| *v).collect(),
            fit.rigs.iter().map(|v| *v).collect(),
            fit.drones.iter().map(|v| *v).collect(),
            fit.fighters.iter().map(|v| *v).collect(),
            fit.fw_effects.iter().map(|v| *v).collect(),
        )
    }
}
