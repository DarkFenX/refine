use itertools::Itertools;

use crate::{
    ac, ad,
    sol::{AttrVal, ItemId, svc::vast::VastFitData, uad::item::Ship},
    util::HSet,
};

pub struct ValCapitalModFail {
    pub max_subcap_volume: AttrVal,
    pub items: Vec<ValCapitalModItemInfo>,
}

pub struct ValCapitalModItemInfo {
    pub item_id: ItemId,
    pub volume: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_fast(
        &self,
        kfs: &HSet<ItemId>,
        ship: Option<&Ship>,
    ) -> bool {
        if !is_ship_subcap(ship) {
            return true;
        }
        match kfs.is_empty() {
            true => self.mods_capital.is_empty(),
            false => self.mods_capital.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_verbose(
        &self,
        kfs: &HSet<ItemId>,
        ship: Option<&Ship>,
    ) -> Option<ValCapitalModFail> {
        if !is_ship_subcap(ship) {
            return None;
        }
        let items = self
            .mods_capital
            .iter()
            .filter(|(k, _)| !kfs.contains(k))
            .map(|(k, v)| ValCapitalModItemInfo {
                item_id: *k,
                volume: *v,
            })
            .collect_vec();
        if items.is_empty() {
            return None;
        }
        Some(ValCapitalModFail {
            max_subcap_volume: ac::extras::MAX_SUBCAP_MODULE_VOLUME,
            items,
        })
    }
}

fn is_ship_subcap(ship: Option<&Ship>) -> bool {
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
