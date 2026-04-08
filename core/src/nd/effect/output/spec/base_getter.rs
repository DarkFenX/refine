use crate::{
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(crate) trait NEffectOutputGetter {
    type Instance: Copy;
    type XArgs;

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        xargs: Self::XArgs,
    ) -> Option<Output<Self::Instance>>;
}
