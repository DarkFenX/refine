use super::traits::{HasImpact, InstanceLimit};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue, UnitInterval},
    rd::{RAttrId, REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Data which stays the same through local effect cycles
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct AggrLocalInvData<T>
where
    T: Copy,
{
    output: Output<T>,
    instance_limit: Option<PValue>,
}
impl<T> AggrLocalInvData<T>
where
    T: Copy + HasImpact,
{
    pub(super) fn try_make<BG, BX>(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        ospec: &REffectLocalOpcSpec<BG>,
        base_xargs: BX,
    ) -> Option<Self>
    where
        BG: NEffectOutputGetter<Instance = T, Xargs = BX>,
    {
        let output = ospec.base.get(ctx, calc, item_uid, effect, base_xargs)?;
        if !output.has_impact() || output.get_instance_count() == Count::ZERO {
            return None;
        }
        Some(AggrLocalInvData {
            output,
            instance_limit: get_ship_limit(ctx, calc, item_uid, ospec.limit_attr_rid),
        })
    }
}

fn get_ship_limit(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, attr_rid: Option<RAttrId>) -> Option<PValue> {
    let attr_rid = attr_rid?;
    let fit_uid = ctx.u_data.items.get(item_uid).get_fit_uid()?;
    let ship_uid = ctx.u_data.fits.get(fit_uid).ship?;
    calc.get_item_attr_oextra(ctx, ship_uid, attr_rid)
        .map(PValue::from_value_clamped)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Converter
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct LocalConverter<'u, 'p, 'c, 'o, 'i, BG, T>
where
    BG: NEffectOutputGetter<Instance = T>,
    T: Copy,
{
    pub(super) ctx: SvcCtx<'u, 'p>,
    pub(super) calc: &'c mut Calc,
    pub(super) item_uid: UItemId,
    pub(super) ospec: &'o REffectLocalOpcSpec<BG>,
    pub(super) inv_local: &'i AggrLocalInvData<T>,
}
impl<'u, 'p, 'c, 'o, 'i, BG, T> LocalConverter<'u, 'p, 'c, 'o, 'i, BG, T>
where
    BG: NEffectOutputGetter<Instance = T>,
    T: Copy,
{
    pub(super) fn new(
        ctx: SvcCtx<'u, 'p>,
        calc: &'c mut Calc,
        item_uid: UItemId,
        ospec: &'o REffectLocalOpcSpec<BG>,
        inv_local: &'i AggrLocalInvData<T>,
    ) -> Self {
        Self {
            ctx,
            calc,
            item_uid,
            ospec,
            inv_local,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_local_output<BG, T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectLocalOpcSpec<BG>,
    inv_local: &AggrLocalInvData<T>,
    chargeness: Option<UnitInterval>,
) -> Output<T>
where
    BG: NEffectOutputGetter<Instance = T>,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
{
    let mut output = inv_local.output;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargeness
        && let Some(charge_mult) = charge_mult_getter.get(ctx, calc, item_uid, chargedness)
    {
        output.instance_mul_assign(charge_mult);
    }
    // Limit
    if let Some(limit) = inv_local.instance_limit {
        output.instance_limit(limit);
    }
    output
}
