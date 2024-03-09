use crate::{defs::SsItemId, ss::SsView, util::Result};

pub(in crate::ss::svc::svce_calc::modifier::custom::prop) fn get_ship_id(
    ss_view: &SsView,
    item_id: &SsItemId,
) -> Result<Option<SsItemId>> {
    let item = ss_view.items.get_item(item_id)?;
    let fit_id = match item.get_fit_id() {
        Some(fit_id) => fit_id,
        None => return Ok(None),
    };
    let fit = ss_view.fits.get_fit(&fit_id)?;
    Ok(fit.ship)
}
