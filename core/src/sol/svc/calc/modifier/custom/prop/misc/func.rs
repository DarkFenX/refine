use crate::sol::{ItemId, uad::Uad};

pub(in crate::sol::svc::calc::modifier::custom::prop) fn get_ship_id(uad: &Uad, item_id: &ItemId) -> Option<ItemId> {
    let item = uad.items.get_item(item_id).unwrap();
    let fit_id = item.get_fit_id()?;
    let fit = uad.fits.get_fit(&fit_id).unwrap();
    fit.ship
}
