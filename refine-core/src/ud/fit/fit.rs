use itertools::chain;

use crate::{
    DpsProfile, FitId, FitSecStatus,
    ad::AItemId,
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
    pub(crate) fn iter_module_uids(&self) -> impl Iterator<Item = UItemId> {
        chain!(
            self.mods_high.iter_uids(),
            self.mods_mid.iter_uids(),
            self.mods_low.iter_uids(),
        )
    }
    pub(crate) fn iter_direct_items(&self) -> impl Iterator<Item = UItemId> {
        chain!(
            self.character,
            self.skills.values().map(|v| v.skill_uid),
            self.implants.iter().copied(),
            self.boosters.iter().copied(),
            self.ship,
            self.stance,
            self.subsystems.iter().copied(),
            self.mods_high.iter_uids(),
            self.mods_mid.iter_uids(),
            self.mods_low.iter_uids(),
            self.rigs.iter().copied(),
            self.services.iter().copied(),
            self.drones.iter().copied(),
            self.fighters.iter().copied(),
            self.fw_effects.iter().copied(),
        )
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
