use crate::{
    ac,
    def::{AttrVal, ItemKey, OF},
    misc::CycleCount,
    svc::{
        SvcCtx,
        calc::Calc,
        efuncs,
        vast::{StatTank, Vast},
    },
    uad::UadItem,
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
        // Buffer - if item is not loaded, fetching those will fail
        let shield_buffer = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::SHIELD_CAPACITY)?;
        let armor_buffer = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::ARMOR_HP)?;
        let structure_buffer = calc.get_item_attr_val_extra(ctx, item_key, &ac::attrs::HP)?;
        // Local ancillary repairs
        let (local_asb, local_aar) = match ctx.uad.items.get(item_key) {
            UadItem::Ship(uad_ship) => {
                let mut local_asb = OF(0.0);
                let mut local_aar = OF(0.0);
                let fit_data = self.get_fit_data(&uad_ship.get_fit_key());
                for (asb_espec, rep_getter) in fit_data.limitable_sb.iter() {
                    if let Some(asb_hp) = rep_getter(ctx, calc, asb_espec.item_key)
                        && let Some(CycleCount::Count(cycles)) = efuncs::get_espec_cycle_count(ctx, *asb_espec)
                    {
                        local_asb += asb_hp * AttrVal::from(cycles);
                    }
                }
                for (aar_espec, rep_getter) in fit_data.limitable_ar.iter() {
                    if let Some(aar_hp) = rep_getter(ctx, calc, aar_espec.item_key)
                        && let Some(CycleCount::Count(cycles)) = efuncs::get_espec_cycle_count(ctx, *aar_espec)
                    {
                        local_aar += aar_hp * AttrVal::from(cycles);
                    }
                }
                (local_asb, local_aar)
            }
            _ => (OF(0.0), OF(0.0)),
        };
        // Incoming remote ancillary repairs
        let mut remote_asb = OF(0.0);
        let mut remote_aar = OF(0.0);
        if let Some(item_limitable_rsbs) = self.limitable_rsb.get_l1(&item_key) {
            for (rasb_espec, rep_getter) in item_limitable_rsbs.iter() {
                if let Some(rasb_hp) = rep_getter(ctx, calc, *rasb_espec, Some(item_key))
                    && let Some(CycleCount::Count(cycles)) = efuncs::get_espec_cycle_count(ctx, *rasb_espec)
                {
                    remote_asb += rasb_hp * AttrVal::from(cycles);
                }
            }
        }
        if let Some(item_limitable_rars) = self.limitable_rar.get_l1(&item_key) {
            for (raar_espec, rep_getter) in item_limitable_rars.iter() {
                if let Some(raar_hp) = rep_getter(ctx, calc, *raar_espec, Some(item_key))
                    && let Some(CycleCount::Count(cycles)) = efuncs::get_espec_cycle_count(ctx, *raar_espec)
                {
                    remote_aar += raar_hp * AttrVal::from(cycles);
                }
            }
        }
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
            structure: StatLayerHp {
                buffer: structure_buffer,
                ancil_local: OF(0.0),
                ancil_remote: OF(0.0),
            },
        })
    }
}
