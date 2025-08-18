use crate::{
    ac,
    def::AttrVal,
    svc::{SvcCtx, calc::Calc, err::KeyedItemLoadedError},
    ud::UItemKey,
};

pub(crate) fn get_speed(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Result<AttrVal, KeyedItemLoadedError> {
    calc.get_item_attr_val_full(ctx, item_key, &ac::attrs::MAX_VELOCITY)
        .map(|v| v.extra)
}

pub(crate) fn get_sig_radius(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Result<AttrVal, KeyedItemLoadedError> {
    calc.get_item_attr_val_full(ctx, item_key, &ac::attrs::SIG_RADIUS)
        .map(|v| v.extra)
}
