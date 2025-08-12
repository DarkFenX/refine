use crate::{
    def::AttrVal,
    misc::{DmgKinds, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
        vast::{StatDmg, Vast, shared::BreacherAccum},
    },
    ud::{UItem, UItemKey},
};

const VOLLEY_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    charged_optionals: false,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_dps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        let mut breacher_accum = BreacherAccum::new();
        let dps_normal = Vast::internal_get_stat_item_dps(
            ctx,
            calc,
            item_key,
            reload,
            spool,
            include_charges,
            ignore_state,
            &mut breacher_accum,
        )?;
        let mut dps = StatDmg::from(dps_normal);
        dps.breacher = breacher_accum.get_dps();
        Ok(dps)
    }
    fn internal_get_stat_item_dps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        breacher_accum: &mut BreacherAccum,
    ) -> Result<DmgKinds<AttrVal>, StatItemCheckError> {
        item_key_check(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_dps_unchecked(
            ctx,
            calc,
            item_key,
            reload,
            spool,
            include_charges,
            ignore_state,
            breacher_accum,
        ))
    }
    fn internal_get_stat_item_dps_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        reload: bool,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
        breacher_accum: &mut BreacherAccum,
    ) -> DmgKinds<AttrVal> {
        let options = CycleOptions {
            reload_mode: match reload {
                true => CycleOptionReload::Sim,
                false => CycleOptionReload::Burst,
            },
            charged_optionals: false,
        };
        let mut item_dps_normal = DmgKinds::new();
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, options, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return item_dps_normal,
        };
        for (effect_key, cycle) in cycle_map {
            if !cycle.is_infinite() {
                continue;
            }
            let effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(dmg_getter) = effect.get_normal_dmg_opc_getter()
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, effect, spool, None)
            {
                item_dps_normal += dmg_opc.get_total() / cycle.get_average_cycle_time();
            }
            if let Some(dmg_getter) = effect.get_breacher_dmg_opc_getter()
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, effect, None)
            {
                breacher_accum.add(dmg_opc, cycle);
            }
        }
        if include_charges {
            for charge_key in ctx.u_data.items.get(item_key).iter_charges() {
                if let Ok(charge_dps_normal) = Vast::internal_get_stat_item_dps(
                    ctx,
                    calc,
                    charge_key,
                    reload,
                    spool,
                    false,
                    ignore_state,
                    breacher_accum,
                ) {
                    item_dps_normal += charge_dps_normal;
                }
            }
        }
        item_dps_normal
    }
    pub(in crate::svc) fn get_stat_item_volley(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> Result<StatDmg, StatItemCheckError> {
        item_key_check(ctx, item_key)?;
        Ok(Vast::internal_get_stat_item_volley_unchecked(
            ctx,
            calc,
            item_key,
            spool,
            include_charges,
            ignore_state,
        ))
    }
    fn internal_get_stat_item_volley_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        include_charges: bool,
        ignore_state: bool,
    ) -> StatDmg {
        let mut item_volley = StatDmg::new();
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, VOLLEY_CYCLE_OPTIONS, ignore_state) {
            Some(cycle_map) => cycle_map,
            None => return item_volley,
        };
        for (effect_key, _cycle) in cycle_map {
            let effect = ctx.u_data.src.get_effect(effect_key);
            if let Some(dmg_getter) = effect.get_normal_dmg_opc_getter()
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, effect, spool, None)
            {
                item_volley.stack_instance_normal(dmg_opc.get_max())
            }
            if let Some(dmg_getter) = effect.get_breacher_dmg_opc_getter()
                && let Some(dmg_opc) = dmg_getter(ctx, calc, item_key, effect, None)
            {
                item_volley.stack_instance_breacher_output(dmg_opc)
            }
        }
        if include_charges {
            for charge_key in ctx.u_data.items.get(item_key).iter_charges() {
                if let Ok(charge_volley) = Vast::get_stat_item_volley(ctx, calc, charge_key, spool, false, ignore_state)
                {
                    item_volley.stack_instance_self(charge_volley);
                }
            }
        }
        item_volley
    }
}

fn item_key_check(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
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
