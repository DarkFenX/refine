use crate::{defs::SolItemId, sol::uad::SolUad};

pub(in crate::sol::svc::calc::modifier::custom::prop) fn get_ship_id(
    uad: &SolUad,
    item_id: &SolItemId,
) -> Option<SolItemId> {
    let item = uad.items.get_item(item_id).unwrap();
    let fit_id = match item.get_fit_id() {
        Some(fit_id) => fit_id,
        None => return None,
    };
    let fit = uad.fits.get_fit(&fit_id).unwrap();
    fit.ship
}
