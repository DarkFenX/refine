use super::shared::get_generic_base_opc;
use crate::{
    nd::NBaseOutputGetter,
    num::PValue,
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(crate) enum NBaseRemoteCapGetter {
    Regular,
}
impl NBaseOutputGetter for NBaseRemoteCapGetter {
    type Instance = PValue;
    type Xargs = ();

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        _xargs: Self::Xargs,
    ) -> Option<Output<Self::Instance>> {
        match self {
            Self::Regular => get_regular(ctx, calc, item_uid, effect),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_regular(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, effect: &REffect) -> Option<Output<PValue>> {
    get_generic_base_opc(ctx, calc, item_uid, effect, ctx.ac().power_transfer_amount, false)
}
