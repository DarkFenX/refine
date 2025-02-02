use crate::{
    ad,
    defs::SolItemId,
    sol::{
        svc::vast::SolVastFitData,
        uad::{fit::SolFit, SolUad},
    },
};

#[derive(Clone)]
pub struct SolCapitalModValFail {
    pub item_id: SolItemId,
}
impl SolCapitalModValFail {
    fn new(item_id: SolItemId) -> Self {
        Self { item_id }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_fast(&self, uad: &SolUad, fit: &SolFit) -> bool {
        is_ship_subcap(uad, fit) && !self.mods_capital.is_empty()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_verbose(
        &self,
        uad: &SolUad,
        fit: &SolFit,
    ) -> Vec<SolCapitalModValFail> {
        if !is_ship_subcap(uad, fit) {
            return Vec::new();
        }
        self.mods_capital
            .iter()
            .map(|v| SolCapitalModValFail::new(*v))
            .collect()
    }
}

fn is_ship_subcap(uad: &SolUad, fit: &SolFit) -> bool {
    let ship = match fit.ship {
        Some(ship_id) => uad.items.get_item(&ship_id).unwrap(),
        None => return false,
    };
    let extras = match ship.get_a_extras() {
        Some(extras) => extras,
        None => return false,
    };
    matches!(extras.kind, Some(ad::AItemKind::Ship(ad::AShipKind::Ship)))
}
