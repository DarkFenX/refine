use crate::{defs::SolItemId, sol::SolView, util::Result};

pub(in crate::sol::svc::svce_calc::modifier::custom::prop) fn get_ship_id(
    sol_view: &SolView,
    item_id: &SolItemId,
) -> Result<Option<SolItemId>> {
    let item = sol_view.items.get_item(item_id)?;
    let fit_id = match item.get_fit_id() {
        Some(fit_id) => fit_id,
        None => return Ok(None),
    };
    let fit = sol_view.fits.get_fit(&fit_id)?;
    Ok(fit.ship)
}
