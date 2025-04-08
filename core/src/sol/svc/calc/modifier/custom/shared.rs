use crate::sol::{ItemKey, uad::Uad};

pub(in crate::sol::svc::calc::modifier::custom) fn get_ship_key(uad: &Uad, item_key: ItemKey) -> Option<ItemKey> {
    let item = uad.items.get(item_key);
    let fit_key = item.get_fit_key()?;
    let fit = uad.fits.get(fit_key);
    fit.ship
}
