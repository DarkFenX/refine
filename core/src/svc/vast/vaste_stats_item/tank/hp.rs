use super::shared::item_check;
use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    misc::EffectSpec,
    nd::{NLocalRepGetter, NRemoteRepGetter},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        misc::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        vast::{StatTank, Vast},
    },
    uad::UadItem,
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
        item_key: ItemKey,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        let uad_item = ctx.uad.items.get(item_key);
        item_check(item_key, uad_item)?;
        Ok(self.get_stat_item_hp_unchecked(ctx, calc, item_key, uad_item))
    }
    pub(super) fn get_stat_item_hp_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        uad_item: &UadItem,
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
        let (local_asb, local_aar) = match uad_item {
            UadItem::Ship(uad_ship) => {
                let fit_data = self.get_fit_data(&uad_ship.get_fit_key());
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
    ancil_data: &RMapRMap<ItemKey, ad::AEffectId, NLocalRepGetter>,
) -> AttrVal {
    let mut total_ancil_hp = OF(0.0);
    for (&item_key, item_data) in ancil_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, ANCIL_CYCLE_OPTIONS, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        for (&a_effect_id, rep_getter) in item_data.iter() {
            let hp_per_cycle = match rep_getter(ctx, calc, item_key) {
                Some(hp_per_cycle) => hp_per_cycle,
                None => continue,
            };
            let effect_cycles = match cycle_map.get(&a_effect_id) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let cycle_count = match effect_cycles.get_cycles_until_reload() {
                InfCount::Count(cycle_count) => cycle_count,
                InfCount::Infinite => continue,
            };
            total_ancil_hp += hp_per_cycle * AttrVal::from(cycle_count);
        }
    }
    total_ancil_hp
}

fn get_remote_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_key: ItemKey,
    ancil_data: &RMapRMapRMap<ItemKey, ItemKey, ad::AEffectId, NRemoteRepGetter>,
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
        for (&a_effect_id, rep_getter) in projector_data.iter() {
            let espec = EffectSpec::new(projector_item_key, a_effect_id);
            let hp_per_cycle = match rep_getter(ctx, calc, espec, None, Some(projectee_item_key)) {
                Some(hp_per_cycle) => hp_per_cycle,
                None => continue,
            };
            let effect_cycles = match projector_cycle_map.get(&a_effect_id) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let cycle_count = match effect_cycles.get_cycles_until_reload() {
                InfCount::Count(cycle_count) => cycle_count,
                InfCount::Infinite => continue,
            };
            total_ancil_hp += hp_per_cycle * AttrVal::from(cycle_count);
        }
    }
    total_ancil_hp
}
