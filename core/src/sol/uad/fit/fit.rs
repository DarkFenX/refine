use crate::{
    defs::{SolFitId, SolFleetId, SolItemId},
    sol::{misc::SolDmgProfile, uad::item::SolShipKind},
    util::StSet,
};

#[derive(Clone)]
pub(in crate::sol) struct SolFit {
    pub(in crate::sol) id: SolFitId,
    pub(in crate::sol) kind: SolShipKind,
    pub(in crate::sol) fleet: Option<SolFleetId>,
    pub(in crate::sol) character: Option<SolItemId>,
    pub(in crate::sol) skills: StSet<SolItemId>,
    pub(in crate::sol) implants: StSet<SolItemId>,
    pub(in crate::sol) boosters: StSet<SolItemId>,
    pub(in crate::sol) ship: Option<SolItemId>,
    pub(in crate::sol) stance: Option<SolItemId>,
    pub(in crate::sol) subsystems: StSet<SolItemId>,
    pub(in crate::sol) mods_high: StSet<SolItemId>,
    pub(in crate::sol) mods_mid: StSet<SolItemId>,
    pub(in crate::sol) mods_low: StSet<SolItemId>,
    pub(in crate::sol) rigs: StSet<SolItemId>,
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
            skills: StSet::new(),
            implants: StSet::new(),
            boosters: StSet::new(),
            ship: None,
            stance: None,
            subsystems: StSet::new(),
            mods_high: StSet::new(),
            mods_mid: StSet::new(),
            mods_low: StSet::new(),
            rigs: StSet::new(),
            drones: StSet::new(),
            fighters: StSet::new(),
            fw_effects: StSet::new(),
            rah_incoming_dmg: None,
        }
    }
    pub(in crate::sol) fn all_items(&self) -> Vec<SolItemId> {
        let mut items = Vec::new();
        conditional_push(&mut items, self.character);
        items.extend(self.skills.iter());
        items.extend(self.implants.iter());
        items.extend(self.boosters.iter());
        conditional_push(&mut items, self.ship);
        conditional_push(&mut items, self.stance);
        items.extend(self.subsystems.iter());
        items.extend(self.mods_high.iter());
        items.extend(self.mods_mid.iter());
        items.extend(self.mods_low.iter());
        items.extend(self.rigs.iter());
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
