use std::collections::HashMap;

use crate::{
    misc::Value,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

pub struct ValUnusableCapFail {
    /// Cap use of any item can't exceed this value.
    pub max_cap: Value,
    /// List of items breaking validation, and their cap uses.
    pub items: HashMap<ItemId, Value>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_unusable_cap_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        ship_uid: Option<UItemId>,
    ) -> bool {
        let ship_uid = match ship_uid {
            Some(ship_uid) => ship_uid,
            None => return true,
        };
        if self.cap_consumers_all.is_empty() {
            return true;
        }
        // Pass validation if ship is not loaded
        let max_cap = match calc.get_item_oattr_afb_oextra(ctx, ship_uid, ctx.ac().capacitor_capacity, Value::ZERO) {
            Some(max_cap) => max_cap,
            None => return true,
        };
        for (&item_uid, attr_rids) in self.cap_consumers_all.iter() {
            for &attr_rid in attr_rids.iter() {
                if let Some(cap_use) = calc.get_item_attr_oextra(ctx, item_uid, attr_rid)
                    && cap_use > max_cap
                    && !kfs.contains(&item_uid)
                {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_unusable_cap_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        ship_uid: Option<UItemId>,
    ) -> Option<ValUnusableCapFail> {
        let ship_uid = ship_uid?;
        if self.cap_consumers_all.is_empty() {
            return None;
        }
        // Pass validation if ship is not loaded
        let max_cap = calc.get_item_oattr_afb_oextra(ctx, ship_uid, ctx.ac().capacitor_capacity, Value::ZERO)?;
        let mut items = HashMap::new();
        for (&item_uid, attr_rids) in self.cap_consumers_all.iter() {
            let max_item_use = match attr_rids
                .iter()
                .filter_map(|&attr_rid| calc.get_item_attr_oextra(ctx, item_uid, attr_rid))
                .max()
            {
                Some(max_item_use) => max_item_use,
                None => continue,
            };
            if max_item_use > max_cap && !kfs.contains(&item_uid) {
                items.insert(ctx.u_data.items.xid_by_iid(item_uid), max_item_use);
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValUnusableCapFail { max_cap, items }),
        }
    }
}
