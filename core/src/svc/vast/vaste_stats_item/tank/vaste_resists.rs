use super::super::checks::check_item_key_drone_fighter_ship;
use crate::{
    def::{AttrVal, OF},
    misc::DmgKinds,
    rd::RAttrKey,
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{StatTank, Vast},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_resists(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatTank<DmgKinds<AttrVal>>, StatItemCheckError> {
        check_item_key_drone_fighter_ship(ctx, item_key)?;
        Ok(Vast::get_stat_item_resists_unchecked(ctx, calc, item_key))
    }
    pub(super) fn get_stat_item_resists_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> StatTank<DmgKinds<AttrVal>> {
        StatTank {
            shield: Vast::get_item_shield_resists(ctx, calc, item_key),
            armor: Vast::get_item_armor_resists(ctx, calc, item_key),
            hull: Vast::get_item_hull_resists(ctx, calc, item_key),
        }
    }
    fn get_item_shield_resists(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> DmgKinds<AttrVal> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            ctx.ac().shield_em_dmg_resonance,
            ctx.ac().shield_therm_dmg_resonance,
            ctx.ac().shield_kin_dmg_resonance,
            ctx.ac().shield_expl_dmg_resonance,
        )
    }
    fn get_item_armor_resists(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> DmgKinds<AttrVal> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            ctx.ac().armor_em_dmg_resonance,
            ctx.ac().armor_therm_dmg_resonance,
            ctx.ac().armor_kin_dmg_resonance,
            ctx.ac().armor_expl_dmg_resonance,
        )
    }
    fn get_item_hull_resists(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> DmgKinds<AttrVal> {
        get_item_layer_resists(
            ctx,
            calc,
            item_key,
            ctx.ac().em_dmg_resonance,
            ctx.ac().therm_dmg_resonance,
            ctx.ac().kin_dmg_resonance,
            ctx.ac().expl_dmg_resonance,
        )
    }
}

fn get_item_layer_resists(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    em_attr_key: Option<RAttrKey>,
    therm_attr_key: Option<RAttrKey>,
    kin_attr_key: Option<RAttrKey>,
    expl_attr_key: Option<RAttrKey>,
) -> DmgKinds<AttrVal> {
    DmgKinds {
        em: OF(1.0)
            - calc
                .get_item_oattr_afb_oextra(ctx, item_key, em_attr_key, OF(1.0))
                .unwrap(),
        thermal: OF(1.0)
            - calc
                .get_item_oattr_afb_oextra(ctx, item_key, therm_attr_key, OF(1.0))
                .unwrap(),
        kinetic: OF(1.0)
            - calc
                .get_item_oattr_afb_oextra(ctx, item_key, kin_attr_key, OF(1.0))
                .unwrap(),
        explosive: OF(1.0)
            - calc
                .get_item_oattr_afb_oextra(ctx, item_key, expl_attr_key, OF(1.0))
                .unwrap(),
    }
}
