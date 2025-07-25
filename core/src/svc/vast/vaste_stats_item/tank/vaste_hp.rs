use super::shared::item_check;
use crate::{
    ac, ad,
    def::{AttrVal, OF},
    nd::{NLocalRepGetter, NRemoteRepGetter},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatTank, Vast},
    },
    ud::{UItem, UItemKey},
    util::{InfCount, RMapRMap, RMapRMapRMap},
};

pub struct StatLayerHp {
    pub buffer: AttrVal,
    pub ancil_local: AttrVal,
    pub ancil_remote: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_hp_checked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        let u_item = ctx.u_data.items.get(item_key);
        item_check(item_key, u_item)?;
        Ok(self.get_stat_item_hp_unchecked(ctx, calc, item_key, u_item))
    }
    pub(super) fn get_stat_item_hp_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        u_item: &UItem,
    ) -> StatTank<StatLayerHp> {
        // Buffer - if item is not loaded, fetching those will fail
        let shield_buffer = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::SHIELD_CAPACITY)
            .unwrap();
        let armor_buffer = calc
            .get_item_attr_val_extra(ctx, item_key, &ac::attrs::ARMOR_HP)
            .unwrap();
        let hull_buffer = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::HP).unwrap();
        // Local ancillary repairs
        let (local_asb, local_aar) = match u_item {
            UItem::Ship(u_ship) => {
                let fit_data = self.get_fit_data(&u_ship.get_fit_key());
                let local_asb = get_local_ancil_hp(ctx, calc, &fit_data.lr_shield_limitable);
                let local_aar = get_local_ancil_hp(ctx, calc, &fit_data.lr_armor_limitable);
                (local_asb, local_aar)
            }
            _ => (OF(0.0), OF(0.0)),
        };
        // Incoming remote ancillary repairs
        let remote_asb = get_remote_ancil_hp(ctx, calc, item_key, &self.irr_shield_limitable);
        let remote_aar = get_remote_ancil_hp(ctx, calc, item_key, &self.irr_armor_limitable);
        StatTank {
            shield: StatLayerHp {
                buffer: shield_buffer,
                ancil_local: local_asb,
                ancil_remote: remote_asb,
            },
            armor: StatLayerHp {
                buffer: armor_buffer,
                ancil_local: local_aar,
                ancil_remote: remote_aar,
            },
            hull: StatLayerHp {
                buffer: hull_buffer,
                ancil_local: OF(0.0),
                ancil_remote: OF(0.0),
            },
        }
    }
}

const ANCIL_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    reload_optionals: true,
};

fn get_local_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    ancil_data: &RMapRMap<UItemKey, ad::AEffectId, NLocalRepGetter>,
) -> AttrVal {
    let mut total_ancil_hp = OF(0.0);
    for (&item_key, item_data) in ancil_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, ANCIL_CYCLE_OPTIONS, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (a_effect_id, rep_getter) in item_data.iter() {
            let effect_cycles = match cycle_map.get(a_effect_id) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let r_effect = ctx.u_data.src.get_r_effect(a_effect_id).unwrap();
            let output_per_cycle = match rep_getter(ctx, calc, item_key, r_effect) {
                Some(hp_per_cycle) => hp_per_cycle,
                None => continue,
            };
            let cycle_count = match effect_cycles.get_cycles_until_reload() {
                InfCount::Count(cycle_count) => cycle_count,
                InfCount::Infinite => continue,
            };
            total_ancil_hp += output_per_cycle.get_total() * cycle_count as f64;
        }
    }
    total_ancil_hp
}

fn get_remote_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_key: UItemKey,
    ancil_data: &RMapRMapRMap<UItemKey, UItemKey, ad::AEffectId, NRemoteRepGetter>,
) -> AttrVal {
    let mut total_ancil_hp = OF(0.0);
    let incoming_ancils = match ancil_data.get_l1(&projectee_item_key) {
        Some(incoming_ancils) => incoming_ancils,
        None => return total_ancil_hp,
    };
    for (&projector_item_key, projector_data) in incoming_ancils.iter() {
        let projector_cycle_map = match get_item_cycle_info(ctx, calc, projector_item_key, ANCIL_CYCLE_OPTIONS, false) {
            Some(projector_cycle_map) => projector_cycle_map,
            None => continue,
        };
        for (a_effect_id, rep_getter) in projector_data.iter() {
            let effect_cycles = match projector_cycle_map.get(a_effect_id) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let r_effect = ctx.u_data.src.get_r_effect(a_effect_id).unwrap();
            let output_per_cycle =
                match rep_getter(ctx, calc, projector_item_key, r_effect, None, Some(projectee_item_key)) {
                    Some(hp_per_cycle) => hp_per_cycle,
                    None => continue,
                };
            let cycle_count = match effect_cycles.get_cycles_until_reload() {
                InfCount::Count(cycle_count) => cycle_count,
                InfCount::Infinite => continue,
            };
            total_ancil_hp += output_per_cycle.get_total() * cycle_count as f64;
        }
    }
    total_ancil_hp
}
