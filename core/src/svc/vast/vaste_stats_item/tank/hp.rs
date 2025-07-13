use super::shared::item_check;
use crate::{
    ac,
    def::{AttrVal, ItemKey, OF},
    misc::{CycleCount, EffectSpec},
    nd::{NLocalRepGetter, NRemoteRepGetter},
    svc::{
        SvcCtx,
        calc::Calc,
        efuncs,
        vast::{StatTank, Vast},
    },
    uad::UadItem,
    util::{RMap, RMapRMap},
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
        item_key: ItemKey,
    ) -> Option<StatTank<StatLayerHp>> {
        let uad_item = ctx.uad.items.get(item_key);
        item_check(uad_item)?;
        // Buffer - if item is not loaded, fetching those will fail
        let shield_buffer = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::SHIELD_CAPACITY)?;
        let armor_buffer = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::ARMOR_HP)?;
        let hull_buffer = calc.get_item_attr_val_extra_opt(ctx, item_key, &ac::attrs::HP)?;
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
        Some(StatTank {
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
        })
    }
}

fn get_local_ancil_hp(ctx: SvcCtx, calc: &mut Calc, ancil_data: &RMap<EffectSpec, NLocalRepGetter>) -> AttrVal {
    let mut total_ancil_hp = OF(0.0);
    for (ancil_espec, rep_getter) in ancil_data.iter() {
        if let Some(ancil_hp) = rep_getter(ctx, calc, ancil_espec.item_key)
            && let Some(CycleCount::Count(cycles)) = efuncs::get_espec_cycle_count(ctx, *ancil_espec)
        {
            total_ancil_hp += ancil_hp * AttrVal::from(cycles);
        }
    }
    total_ancil_hp
}

fn get_remote_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    ancil_data: &RMapRMap<ItemKey, EffectSpec, NRemoteRepGetter>,
) -> AttrVal {
    let mut total_ancil_hp = OF(0.0);
    if let Some(incoming_ancils) = ancil_data.get_l1(&item_key) {
        for (ancil_espec, rep_getter) in incoming_ancils.iter() {
            if let Some(ancil_hp) = rep_getter(ctx, calc, *ancil_espec, None, Some(item_key))
                && let Some(CycleCount::Count(cycles)) = efuncs::get_espec_cycle_count(ctx, *ancil_espec)
            {
                total_ancil_hp += ancil_hp * AttrVal::from(cycles);
            }
        }
    }
    total_ancil_hp
}
