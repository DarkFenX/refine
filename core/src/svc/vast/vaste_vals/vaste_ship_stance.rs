use crate::{
    ac,
    def::{ItemId, ItemKey},
    svc::{SvcCtx, vast::VastFitData},
    uad::{UadFit, UadShip},
    util::RSet,
};

/// Fails when a ship which can't have a stance has one.
pub struct ValShipStanceFail {
    /// Stance item ID.
    pub stance_item_id: ItemId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_ship_stance_fast(
        &self,
        kfs: &RSet<ItemKey>,
        fit: &UadFit,
        ship: Option<&UadShip>,
    ) -> bool {
        let stance_key = match fit.stance {
            Some(stance_key) => stance_key,
            None => return true,
        };
        let ship = match ship {
            Some(ship) => ship,
            None => return false,
        };
        matches!(
            ship.get_a_item_id(),
            ac::items::CONFESSOR | ac::items::HECATE | ac::items::JACKDAW | ac::items::SVIPUL
        ) || kfs.contains(&stance_key)
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_ship_stance_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
        fit: &UadFit,
        ship: Option<&UadShip>,
    ) -> Option<ValShipStanceFail> {
        let stance_key = fit.stance?;
        let ship = match ship {
            Some(ship) => ship,
            None => {
                return Some(ValShipStanceFail {
                    stance_item_id: ctx.uad.items.id_by_key(stance_key),
                });
            }
        };
        if matches!(
            ship.get_a_item_id(),
            ac::items::CONFESSOR | ac::items::HECATE | ac::items::JACKDAW | ac::items::SVIPUL
        ) {
            return None;
        }
        if kfs.contains(&stance_key) {
            return None;
        }
        Some(ValShipStanceFail {
            stance_item_id: ctx.uad.items.id_by_key(stance_key),
        })
    }
}
