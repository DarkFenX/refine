use crate::{
    def::{AttrVal, OF},
    nd::{NEffectLocalOpcSpec, NEffectProjOpcSpec},
    rd::REffectKey,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptions, CycleOptionsSim, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatTank, Vast, vaste_stats::item_checks::check_drone_fighter_ship},
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
    pub(in crate::svc) fn get_stat_item_hp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_key)?;
        Ok(self.get_stat_item_hp_unchecked(ctx, calc, item_key, item))
    }
    pub(super) fn get_stat_item_hp_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        item: &UItem,
    ) -> StatTank<StatLayerHp> {
        let attr_consts = ctx.ac();
        // Buffer - if item is not loaded, fetching those will fail
        let shield_buffer = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.shield_capacity, OF(0.0))
            .unwrap();
        let armor_buffer = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.armor_hp, OF(0.0))
            .unwrap();
        let hull_buffer = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.hp, OF(0.0))
            .unwrap();
        // Local ancillary repairs
        let (local_asb, local_aar) = match item {
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

const ANCIL_CYCLE_OPTIONS: CycleOptions = CycleOptions::Sim(CycleOptionsSim {
    reload_optionals: Some(true),
    ..
});

fn get_local_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    ancil_data: &RMapRMap<UItemKey, REffectKey, NEffectLocalOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut total_ancil_hp = OF(0.0);
    for (&item_key, item_data) in ancil_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, ANCIL_CYCLE_OPTIONS, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        'effect: for (&effect_key, rep_ospec) in item_data.iter() {
            let effect_cycles = match cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            let mut broken_sequence = false;
            let mut effect_ancil_hp = OF(0.0);
            let effect_cycle_parts = effect_cycles.get_parts();
            for effect_cycle_part in effect_cycle_parts.iter() {
                // No charges in current part breaks sequence
                if effect_cycle_part.data.chargedness.is_none() {
                    broken_sequence = true;
                    break;
                }
                let effect_part_repeats = match effect_cycle_part.repeat_count {
                    InfCount::Count(effect_part_repeats) => effect_part_repeats,
                    InfCount::Infinite => match effect_cycle_part.data.interrupt {
                        // Infinite cycle with reload marker means it has to reload every cycle,
                        // which is acceptable
                        Some(interrupt) if interrupt.reload => 1,
                        // Can infinitely cycle without reloads - current effect is not an ancil,
                        // skip it completely
                        _ => continue 'effect,
                    },
                };
                let hp_per_cycle =
                    match rep_ospec.get_total(ctx, calc, item_key, effect, effect_cycle_part.data.chargedness) {
                        Some(hp_per_cycle) => hp_per_cycle,
                        // Assume that if HP was not returned for this part, it cannot be returned for
                        // this effect altogether
                        None => continue 'effect,
                    };
                effect_ancil_hp += hp_per_cycle * effect_part_repeats as f64;
                // Reloads break sequence
                if let Some(interrupt) = effect_cycle_part.data.interrupt
                    && interrupt.reload
                {
                    broken_sequence = true;
                    break;
                }
            }
            // If cycle was not broken early, and it loops, it is infinitely cycling, and thus not
            // an ancil
            if !broken_sequence && effect_cycle_parts.loops {
                continue;
            }
            // Add HP only after we concluded it is an ancil
            total_ancil_hp += effect_ancil_hp;
        }
    }
    total_ancil_hp
}

fn get_remote_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_key: UItemKey,
    ancil_data: &RMapRMapRMap<UItemKey, UItemKey, REffectKey, NEffectProjOpcSpec<AttrVal>>,
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
        'effect: for (&effect_key, ospec) in projector_data.iter() {
            let effect_cycles = match projector_cycle_map.get(&effect_key) {
                Some(effect_cycles) => effect_cycles,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            let mut broken_sequence = false;
            let mut effect_ancil_hp = OF(0.0);
            let effect_cycle_parts = effect_cycles.get_parts();
            for effect_cycle_part in effect_cycle_parts.iter() {
                // No charges in current part breaks sequence
                if effect_cycle_part.data.chargedness.is_none() {
                    broken_sequence = true;
                    break;
                }
                let effect_part_repeats = match effect_cycle_part.repeat_count {
                    InfCount::Count(effect_part_repeats) => effect_part_repeats,
                    InfCount::Infinite => match effect_cycle_part.data.interrupt {
                        // Infinite cycle with reload marker means it has to reload every cycle,
                        // which is acceptable
                        Some(interrupt) if interrupt.reload => 1,
                        // Can infinitely cycle without reloads - current effect is not an ancil,
                        // skip it completely
                        _ => continue 'effect,
                    },
                };
                let hp_per_cycle = match ospec.get_total(
                    ctx,
                    calc,
                    projector_item_key,
                    effect,
                    effect_cycle_part.data.chargedness,
                    None,
                    Some(projectee_item_key),
                ) {
                    Some(hp_per_cycle) => hp_per_cycle,
                    // Assume that if HP was not returned for this part, it cannot be returned for
                    // this effect altogether
                    None => continue 'effect,
                };
                effect_ancil_hp += hp_per_cycle * effect_part_repeats as f64;
                // Reloads break sequence
                if let Some(interrupt) = effect_cycle_part.data.interrupt
                    && interrupt.reload
                {
                    broken_sequence = true;
                    break;
                }
            }
            // If cycle was not broken early, and it loops, it is infinitely cycling, and thus not
            // an ancil
            if !broken_sequence && effect_cycle_parts.loops {
                continue;
            }
            // Add HP only after we concluded it is an ancil
            total_ancil_hp += effect_ancil_hp;
        }
    }
    total_ancil_hp
}
