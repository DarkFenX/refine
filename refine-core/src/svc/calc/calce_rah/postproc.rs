use crate::{
    misc::DmgKinds,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVals},
    },
    ud::UItemId,
};

impl Calc {
    fn get_rah_resonances(&mut self, ctx: SvcCtx, item_uid: UItemId) -> DmgKinds<CalcAttrVals> {
        // Unwrap item, since method is supposed to be called only for registered RAHs
        if let Some(val) = self.rah.resonances.get(&item_uid).unwrap() {
            return *val;
        }
        // Unwrap fit ID, since registered RAHs are supposed to be modules, which have fit ID
        let fit_uid = ctx.u_data.items.get(item_uid).get_fit_uid().unwrap();
        self.rah.sim_running = true;
        self.rah_run_simulation(ctx, fit_uid);
        self.rah.sim_running = false;
        // Unwrap value, since simulation is supposed to always set results for RAHs of requested
        // fit
        self.rah.resonances.get(&item_uid).unwrap().unwrap()
    }
}

pub(in crate::svc::calc) fn rah_em_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
) -> CalcAttrVals {
    calc.get_rah_resonances(ctx, item_uid).em
}
pub(in crate::svc::calc) fn rah_therm_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
) -> CalcAttrVals {
    calc.get_rah_resonances(ctx, item_uid).thermal
}
pub(in crate::svc::calc) fn rah_kin_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
) -> CalcAttrVals {
    calc.get_rah_resonances(ctx, item_uid).kinetic
}
pub(in crate::svc::calc) fn rah_expl_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
) -> CalcAttrVals {
    calc.get_rah_resonances(ctx, item_uid).explosive
}

pub(in crate::svc::calc) fn rah_em_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_uid).em.extra;
    info
}
pub(in crate::svc::calc) fn rah_therm_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_uid).thermal.extra;
    info
}
pub(in crate::svc::calc) fn rah_kin_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_uid).kinetic.extra;
    info
}
pub(in crate::svc::calc) fn rah_expl_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_uid: UItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_uid).explosive.extra;
    info
}
