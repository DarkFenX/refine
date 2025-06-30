use crate::{
    ac, ad,
    def::{AttrVal, ItemKey, OF},
    misc::DmgKinds,
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{
            Vast,
            vaste_stats_effect::{
                get_effect_charge, get_effect_local_armor_rep_amount, get_effect_local_shield_rep_amount,
                get_effect_remote_armor_rep_amount, get_effect_remote_shield_rep_amount,
            },
        },
    },
    uad::UadItem,
};

pub struct StatTank<T> {
    pub shield: T,
    pub armor: T,
    pub structure: T,
}

pub struct StatLayerHp {
    pub buffer: AttrVal,
    pub ancil_local: AttrVal,
    pub ancil_remote: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_hp(
        &self,
        ctx: &SvcCtx,
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
                for asb_espec in fit_data.limitable_sb.iter() {
                    if let Some(asb_hp) = get_effect_local_shield_rep_amount(ctx, calc, asb_espec)
                        && let Some(cycles) = get_effect_charge(ctx, asb_espec).get_cycle_count()
                    {
                        local_asb += asb_hp * AttrVal::from(cycles);
                    }
                }
                for aar_espec in fit_data.limitable_ar.iter() {
                    if let Some(aar_hp) = get_effect_local_armor_rep_amount(ctx, calc, aar_espec)
                        && let Some(cycles) = get_effect_charge(ctx, aar_espec).get_cycle_count()
                    {
                        local_aar += aar_hp * AttrVal::from(cycles);
                    }
                }
                (local_asb, local_aar)
            }
            _ => (OF(0.0), OF(0.0)),
        };
        // Remote ancillary repairs
        let mut remote_asb = OF(0.0);
        let mut remote_aar = OF(0.0);
        for rasb_espec in self.limitable_rsb.get(&item_key) {
            if let Some(rasb_hp) = get_effect_remote_shield_rep_amount(ctx, calc, rasb_espec, Some(item_key))
                && let Some(cycles) = get_effect_charge(ctx, rasb_espec).get_cycle_count()
            {
                remote_asb += rasb_hp * AttrVal::from(cycles);
            }
        }
        for raar_espec in self.limitable_rar.get(&item_key) {
            if let Some(raar_hp) = get_effect_remote_armor_rep_amount(ctx, calc, raar_espec, Some(item_key))
                && let Some(cycles) = get_effect_charge(ctx, raar_espec).get_cycle_count()
            {
                remote_aar += raar_hp * AttrVal::from(cycles);
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
    pub(in crate::svc) fn get_stat_item_resists(
        ctx: &SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<StatTank<DmgKinds<AttrVal>>> {
        Some(StatTank {
            shield: Vast::get_item_shield_resists(ctx, calc, item_key)?,
            armor: Vast::get_item_armor_resists(ctx, calc, item_key)?,
            structure: Vast::get_item_structure_resists(ctx, calc, item_key)?,
        })
    }
    pub(in crate::svc) fn get_item_shield_resists(
        ctx: &SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<DmgKinds<AttrVal>> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            &ac::attrs::SHIELD_EM_DMG_RESONANCE,
            &ac::attrs::SHIELD_THERM_DMG_RESONANCE,
            &ac::attrs::SHIELD_KIN_DMG_RESONANCE,
            &ac::attrs::SHIELD_EXPL_DMG_RESONANCE,
        )
    }
    pub(in crate::svc) fn get_item_armor_resists(
        ctx: &SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<DmgKinds<AttrVal>> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            &ac::attrs::ARMOR_EM_DMG_RESONANCE,
            &ac::attrs::ARMOR_THERM_DMG_RESONANCE,
            &ac::attrs::ARMOR_KIN_DMG_RESONANCE,
            &ac::attrs::ARMOR_EXPL_DMG_RESONANCE,
        )
    }
    pub(in crate::svc) fn get_item_structure_resists(
        ctx: &SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<DmgKinds<AttrVal>> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            &ac::attrs::EM_DMG_RESONANCE,
            &ac::attrs::THERM_DMG_RESONANCE,
            &ac::attrs::KIN_DMG_RESONANCE,
            &ac::attrs::EXPL_DMG_RESONANCE,
        )
    }
}

fn get_item_layer_resists(
    ctx: &SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    em_a_attr_id: &ad::AAttrId,
    therm_a_attr_id: &ad::AAttrId,
    kin_a_attr_id: &ad::AAttrId,
    expl_a_attr_id: &ad::AAttrId,
) -> Option<DmgKinds<AttrVal>> {
    Some(DmgKinds {
        em: calc.get_item_attr_val_extra(ctx, item_key, em_a_attr_id)?,
        thermal: calc.get_item_attr_val_extra(ctx, item_key, therm_a_attr_id)?,
        kinetic: calc.get_item_attr_val_extra(ctx, item_key, kin_a_attr_id)?,
        explosive: calc.get_item_attr_val_extra(ctx, item_key, expl_a_attr_id)?,
    })
}
