use crate::{
    def::AttrVal,
    misc::{DmgKinds, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        vast::Vast,
    },
    ud::{UItem, UItemKey},
};

const VOLLEY_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    reload_optionals: false,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_dps_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<DmgKinds<AttrVal>, StatItemCheckError> {
        item_key_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_dps_unchecked(
            ctx,
            calc,
            item_key,
            reload,
            spool,
            ignore_state,
        ))
    }
    fn get_stat_item_dps_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> DmgKinds<AttrVal> {
        let options = CycleOptions {
            reload_mode: match reload {
                true => CycleOptionReload::Sim,
                false => CycleOptionReload::Burst,
            },
            reload_optionals: false,
        };
        let mut item_dps = DmgKinds::new();
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, options, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return item_dps,
        };
        for (a_effect_id, cycle) in cycle_map {
            let r_effect = ctx.u_data.src.get_r_effect(&a_effect_id).unwrap();
            if let Some(dmg_getter) = r_effect.get_normal_dmg_opc_getter()
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, r_effect, spool, None)
            {
                item_dps += dmg_opc.get_total() / cycle.get_average_cycle_time();
            }
        }
        item_dps
    }
    pub(in crate::svc) fn get_stat_item_volley_checked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<DmgKinds<AttrVal>, StatItemCheckError> {
        item_key_check(ctx, item_key)?;
        Ok(Vast::get_stat_item_volley_unchecked(
            ctx,
            calc,
            item_key,
            spool,
            ignore_state,
        ))
    }
    fn get_stat_item_volley_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> DmgKinds<AttrVal> {
        let mut item_volley = DmgKinds::new();
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, VOLLEY_CYCLE_OPTIONS, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return item_volley,
        };
        for (a_effect_id, _cycle) in cycle_map {
            let r_effect = ctx.u_data.src.get_r_effect(&a_effect_id).unwrap();
            if let Some(dmg_getter) = r_effect.get_normal_dmg_opc_getter()
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, r_effect, spool, None)
            {
                item_volley += dmg_opc.get_max()
            }
        }
        item_volley
    }
}

pub(super) fn item_key_check(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Autocharge(autocharge) => autocharge.is_loaded(),
        UItem::Charge(charge) => charge.is_loaded(),
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
