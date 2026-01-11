use super::stat::{StatResists, StatResistsLayer};
use crate::{
    num::{UnitInterval, Value},
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{Vast, stats::item_checks::check_drone_fighter_ship},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_resists(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<StatResists, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(Vast::get_stat_item_resists_unchecked(ctx, calc, item_uid))
    }
    pub(in crate::svc::vast::stats::tank) fn get_stat_item_resists_unchecked(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> StatResists {
        StatResists {
            shield: Vast::get_item_shield_resists(ctx, calc, item_uid),
            armor: Vast::get_item_armor_resists(ctx, calc, item_uid),
            hull: Vast::get_item_hull_resists(ctx, calc, item_uid),
        }
    }
    fn get_item_shield_resists(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> StatResistsLayer {
        get_item_layer_resists(
            ctx,
            calc,
            item_uid,
            ctx.ac().shield_em_dmg_resonance,
            ctx.ac().shield_therm_dmg_resonance,
            ctx.ac().shield_kin_dmg_resonance,
            ctx.ac().shield_expl_dmg_resonance,
        )
    }
    fn get_item_armor_resists(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> StatResistsLayer {
        get_item_layer_resists(
            ctx,
            calc,
            item_uid,
            ctx.ac().armor_em_dmg_resonance,
            ctx.ac().armor_therm_dmg_resonance,
            ctx.ac().armor_kin_dmg_resonance,
            ctx.ac().armor_expl_dmg_resonance,
        )
    }
    fn get_item_hull_resists(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> StatResistsLayer {
        get_item_layer_resists(
            ctx,
            calc,
            item_uid,
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
    item_uid: UItemId,
    em_attr_rid: Option<RAttrId>,
    therm_attr_rid: Option<RAttrId>,
    kin_attr_rid: Option<RAttrId>,
    expl_attr_rid: Option<RAttrId>,
) -> StatResistsLayer {
    StatResistsLayer {
        em: UnitInterval::from_value_clamped(
            Value::ONE
                - calc
                    .get_item_oattr_afb_oextra(ctx, item_uid, em_attr_rid, Value::ONE)
                    .unwrap(),
        ),
        thermal: UnitInterval::from_value_clamped(
            Value::ONE
                - calc
                    .get_item_oattr_afb_oextra(ctx, item_uid, therm_attr_rid, Value::ONE)
                    .unwrap(),
        ),
        kinetic: UnitInterval::from_value_clamped(
            Value::ONE
                - calc
                    .get_item_oattr_afb_oextra(ctx, item_uid, kin_attr_rid, Value::ONE)
                    .unwrap(),
        ),
        explosive: UnitInterval::from_value_clamped(
            Value::ONE
                - calc
                    .get_item_oattr_afb_oextra(ctx, item_uid, expl_attr_rid, Value::ONE)
                    .unwrap(),
        ),
    }
}
