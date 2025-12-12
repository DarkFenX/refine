use std::collections::HashMap;

use crate::{
    def::{AttrVal, ItemId, OF},
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::UItemKey,
    util::RSet,
};

pub struct ValUnusableCapFail {
    /// Cap use of any item can't exceed this value.
    pub max_cap: AttrVal,
    /// List of items breaking validation, and their cap uses.
    pub items: HashMap<ItemId, AttrVal>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_unusable_cap_fast(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        ship_key: Option<UItemKey>,
    ) -> bool {
        let ship_key = match ship_key {
            Some(ship_key) => ship_key,
            None => return true,
        };
        if self.cap_consumers.is_empty() {
            return true;
        }
        // Pass validation if ship is not loaded
        let max_cap = match calc.get_item_oattr_afb_oextra(ctx, ship_key, ctx.ac().capacitor_capacity, OF(0.0)) {
            Some(max_cap) => max_cap,
            None => return true,
        };
        for (&item_key, item_data) in self.cap_consumers.iter() {
            for &attr_key in item_data.values() {
                if let Some(cap_use) = calc.get_item_attr_oextra(ctx, item_key, attr_key)
                    && cap_use > max_cap
                    && !kfs.contains(&item_key)
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
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
        ship_key: Option<UItemKey>,
    ) -> Option<ValUnusableCapFail> {
        let ship_key = ship_key?;
        if self.cap_consumers.is_empty() {
            return None;
        }
        // Pass validation if ship is not loaded
        let max_cap = match calc.get_item_oattr_afb_oextra(ctx, ship_key, ctx.ac().capacitor_capacity, OF(0.0)) {
            Some(max_cap) => max_cap,
            None => return None,
        };
        let mut items = HashMap::new();
        for (&item_key, item_data) in self.cap_consumers.iter() {
            let max_item_use = match item_data
                .values()
                .filter_map(|&attr_key| calc.get_item_attr_oextra(ctx, item_key, attr_key))
                .max()
            {
                Some(max_item_use) => max_item_use,
                None => continue,
            };
            if max_item_use > max_cap && !kfs.contains(&item_key) {
                items.insert(ctx.u_data.items.id_by_key(item_key), max_item_use);
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValUnusableCapFail { max_cap, items }),
        }
    }
}
