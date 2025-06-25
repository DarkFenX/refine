use smallvec::SmallVec;

use super::attr::{PROP_BOOST, PROP_THRUST, SHIP_MASS};
use crate::sol::{
    ItemKey,
    svc::{
        SvcCtx,
        calc::{AffectorInfo, modifier::custom::shared::get_ship_key},
    },
};

pub(in crate::sol::svc::calc::modifier) fn get_affector_info(
    ctx: &SvcCtx,
    item_key: ItemKey,
) -> SmallVec<AffectorInfo, 1> {
    let mut affectors = SmallVec::new();
    if let Some(ship_key) = get_ship_key(ctx, item_key) {
        let item_id = ctx.uad.items.id_by_key(item_key);
        affectors.push(AffectorInfo {
            item_id,
            attr_id: Some(PROP_BOOST),
        });
        affectors.push(AffectorInfo {
            item_id,
            attr_id: Some(PROP_THRUST),
        });
        affectors.push(AffectorInfo {
            item_id: ctx.uad.items.id_by_key(ship_key),
            attr_id: Some(SHIP_MASS),
        });
    }
    affectors
}
