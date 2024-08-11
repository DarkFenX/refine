use crate::{defs::SolItemId, sol::SolView};

pub(in crate::sol::svc::svce_calc::modifier::custom::prop) fn get_ship_id(
    sol_view: &SolView,
    item_id: &SolItemId,
) -> Option<SolItemId> {
    let item = sol_view.items.get_item(item_id).unwrap();
    let fit_id = match item.get_fit_id() {
        Some(fit_id) => fit_id,
        None => return None,
    };
    let fit = sol_view.fits.get_fit(&fit_id).unwrap();
    fit.ship
}
