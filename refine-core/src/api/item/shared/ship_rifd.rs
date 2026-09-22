use crate::{
    rd::RItemFlexData,
    ud::{UData, UFitId},
};

pub(in crate::api) fn get_ship_rifd(u_data: &UData, fit_uid: UFitId) -> Option<&RItemFlexData> {
    let u_fit = u_data.fits.get(fit_uid);
    let ship_uid = u_fit.ship?;
    let u_ship = u_data.items.get(ship_uid);
    u_ship.get_r_item_flex_data()
}
