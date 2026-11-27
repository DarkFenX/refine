use crate::{
    Value,
    svc::{
        Calc, SvcCtx, Vast,
        err::IntStatItemError,
        vast::stats::item_checks::{check_module_service, check_rig},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_cpu_use(
        ctx: &SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Value, IntStatItemError<!>> {
        check_module_service(ctx.u_data, item_uid)?;
        Ok(calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().cpu, Value::ZERO))
    }
    pub(in crate::svc) fn get_stat_item_powergrid_use(
        ctx: &SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<Value, IntStatItemError<!>> {
        check_module_service(ctx.u_data, item_uid)?;
        Ok(calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().power, Value::ZERO))
    }
    pub(in crate::svc) fn get_stat_item_calibration_use(
        ctx: &SvcCtx,
        item_uid: UItemId,
    ) -> Result<Value, IntStatItemError<!>> {
        let rig = check_rig(ctx.u_data, item_uid)?;
        Ok(rig.get_r_item_base().unwrap().calibration_use.unwrap_or(Value::ZERO))
    }
}
