use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        vast::Vast,
    },
    ud::{UItem, UItemKey},
};

const NEUT_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    charged_optionals: false,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_remote_nps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        ignore_state: bool,
    ) -> Result<AttrVal, StatItemCheckError> {
        item_check_neuting(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_remote_nps_unchecked(
            ctx,
            calc,
            item_key,
            ignore_state,
        ))
    }
    fn internal_get_stat_item_remote_nps_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        ignore_state: bool,
    ) -> AttrVal {
        let mut item_nps = OF(0.0);
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, NEUT_CYCLE_OPTIONS, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return item_nps,
        };
        for (effect_key, effect_cycle) in cycle_map {
            if !effect_cycle.is_infinite() {
                continue;
            }
            let r_effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(neut_getter) = r_effect.get_neut_opc_getter()
                && let Some(neut_amount) = neut_getter(ctx, calc, item_key, &r_effect, None)
            {
                item_nps += neut_amount.get_total() / effect_cycle.get_average_cycle_time();
            }
        }
        item_nps
    }
}

fn item_check_neuting(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}
