use crate::{
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(crate) trait NBaseOutputGetter {
    type Instance: Copy;
    type Xargs;

    fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        xargs: Self::Xargs,
    ) -> Option<Output<Self::Instance>>;
}
