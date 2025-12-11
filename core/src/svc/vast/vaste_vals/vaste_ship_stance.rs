use crate::{
    ac,
    def::ItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UFit, UItemKey, UShip},
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
        kfs: &RSet<UItemKey>,
        fit: &UFit,
        ship: Option<&UShip>,
    ) -> bool {
        let stance_key = match fit.stance {
            Some(stance_key) => stance_key,
            None => return true,
        };
        let ship = match ship {
            Some(ship) => ship,
            None => return false,
        };
        stanceable_matcher(ship) || kfs.contains(&stance_key)
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_ship_stance_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        fit: &UFit,
        ship: Option<&UShip>,
    ) -> Option<ValShipStanceFail> {
        let stance_key = fit.stance?;
        let ship = match ship {
            Some(ship) => ship,
            None => {
                return Some(ValShipStanceFail {
                    stance_item_id: ctx.u_data.items.id_by_key(stance_key),
                });
            }
        };
        if stanceable_matcher(ship) {
            return None;
        }
        if kfs.contains(&stance_key) {
            return None;
        }
        Some(ValShipStanceFail {
            stance_item_id: ctx.u_data.items.id_by_key(stance_key),
        })
    }
}

fn stanceable_matcher(ship: &UShip) -> bool {
    matches!(
        ship.get_type_id(),
        ac::items::CONFESSOR
            | ac::items::HECATE
            | ac::items::JACKDAW
            | ac::items::SVIPUL
            | ac::items::SKUA
            | ac::items::ANHINGA
    )
}
