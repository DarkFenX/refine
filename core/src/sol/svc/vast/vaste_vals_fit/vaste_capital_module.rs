use crate::{
    ad,
    defs::{AttrVal, SolItemId},
    ec,
    sol::{svc::vast::SolVastFitData, uad::item::SolShip},
};

pub struct SolCapitalModValFail {
    pub max_subcap_volume: AttrVal,
    pub items: Vec<SolCapitalModItemInfo>,
}
impl SolCapitalModValFail {
    fn new(max_subcap_volume: AttrVal, items: Vec<SolCapitalModItemInfo>) -> Self {
        Self {
            max_subcap_volume,
            items,
        }
    }
}

#[derive(Copy, Clone)]
pub struct SolCapitalModItemInfo {
    pub item_id: SolItemId,
    pub volume: AttrVal,
}
impl SolCapitalModItemInfo {
    pub(in crate::sol::svc::vast) fn new(item_id: SolItemId, volume: AttrVal) -> Self {
        Self { item_id, volume }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_fast(&self, ship: Option<&SolShip>) -> bool {
        !is_ship_subcap(ship) || self.mods_capital.is_empty()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_verbose(
        &self,
        ship: Option<&SolShip>,
    ) -> Option<SolCapitalModValFail> {
        if !is_ship_subcap(ship) || self.mods_capital.is_empty() {
            return None;
        }
        Some(SolCapitalModValFail::new(
            ec::extras::MAX_SUBCAP_MODULE_VOLUME,
            self.mods_capital.values().copied().collect(),
        ))
    }
}

fn is_ship_subcap(ship: Option<&SolShip>) -> bool {
    let ship = match ship {
        Some(ship) => ship,
        None => return false,
    };
    let extras = match ship.get_a_extras() {
        Some(extras) => extras,
        None => return false,
    };
    matches!(extras.ship_kind, Some(ad::AShipKind::Ship))
}
