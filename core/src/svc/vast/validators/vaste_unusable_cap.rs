use std::collections::HashMap;

use crate::{
    nd::NEffectOutputGetter,
    num::{PValue, UnitInterval, Value},
    rd::RItemCapConsumer,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{ItemId, UItem, UItemId},
    util::RSet,
};

pub struct ValUnusableCapFail {
    /// Cap use of any item can't exceed this value.
    pub max_cap: PValue,
    /// List of items breaking validation, and their cap uses.
    pub items: HashMap<ItemId, PValue>,
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
        let Some(ship_uid) = ship_uid else {
            return true;
        };
        if self.mods_cap_consumers.is_empty() {
            return true;
        }
        // Pass validation if ship is not loaded
        let max_cap = match calc.get_item_oattr_afb_oextra(ctx, ship_uid, ctx.ac().capacitor_capacity, Value::ZERO) {
            Some(max_cap) => PValue::from_value_clamped(max_cap),
            None => return true,
        };
        for &item_uid in self.mods_cap_consumers.iter() {
            let u_item = ctx.u_data.items.get(item_uid);
            for cap_consumer in u_item.get_cap_consumers().unwrap().iter() {
                let Some(cap_consumed) = get_cap_consumption_instance(ctx, calc, item_uid, u_item, cap_consumer) else {
                    continue;
                };
                if cap_consumed > max_cap && !kfs.contains(&item_uid) {
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
        if self.mods_cap_consumers.is_empty() {
            return None;
        }
        // Pass validation if ship is not loaded
        let max_cap = calc.get_item_oattr_afb_oextra(ctx, ship_uid, ctx.ac().capacitor_capacity, Value::ZERO)?;
        let max_cap = PValue::from_value_clamped(max_cap);
        let mut items = HashMap::new();
        for &item_uid in self.mods_cap_consumers.iter() {
            let u_item = ctx.u_data.items.get(item_uid);
            let Some(max_item_use) = u_item
                .get_cap_consumers()
                .unwrap()
                .iter()
                .filter_map(|cap_consumer| get_cap_consumption_instance(ctx, calc, item_uid, u_item, cap_consumer))
                .max()
            else {
                continue;
            };
            if max_item_use > max_cap && !kfs.contains(&item_uid) {
                items.insert(ctx.u_data.items.ext_id_by_int_id(item_uid), max_item_use);
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValUnusableCapFail { max_cap, items }),
        }
    }
}

fn get_cap_consumption_instance(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    u_item: &UItem,
    cap_consumer: &RItemCapConsumer,
) -> Option<PValue> {
    let r_effect = ctx.u_data.src.get_effect_by_rid(cap_consumer.effect_rid);
    let mut cap_consumed = cap_consumer
        .opc_spec
        .base
        .get(ctx, calc, item_uid, r_effect, ())?
        .get_instance();
    // Just assume chargedness is 100% when there is a charge for simplicity. The only case it's
    // needed in this case (ASBs) chargedness is ignored anyway.
    if let Some(charge_mult_getter) = cap_consumer.opc_spec.charge_mult
        && u_item.get_charge_uid().is_some()
        && let Some(charge_mult) = charge_mult_getter.get(ctx, calc, item_uid, UnitInterval::ONE)
    {
        cap_consumed *= charge_mult;
    }
    Some(cap_consumed)
}
