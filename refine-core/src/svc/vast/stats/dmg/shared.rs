use crate::{
    PValue,
    misc::DmgKinds,
    nd::NEffectDmgOutputGetter,
    rd::{REffect, REffectProjOpcSpec},
    stats::{StatCritOptions, StatTimeOptions},
    svc::{
        Calc, SvcCtx,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
        vast::aggr::{SeqAccum, SeqInstanceAccumMax, aggr_proj_burst, aggr_proj_split, aggr_proj_time},
    },
    ud::UItemId,
};

pub(super) fn add_normal_dmg(
    ctx: &SvcCtx,
    calc: &mut Calc,
    dps_normal: &mut DmgKinds<PValue>,
    volley_normal: &mut DmgKinds<PValue>,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<NEffectDmgOutputGetter>,
    time_options: StatTimeOptions,
    crit_options: StatCritOptions,
    projectee_uid: Option<UItemId>,
) {
    match time_options {
        StatTimeOptions::Burst(burst_opts) => {
            if let Some(accum) = aggr_proj_burst(
                ctx,
                calc,
                item_uid,
                effect,
                cseq,
                ospec,
                (),
                crit_options,
                projectee_uid,
                burst_opts.spool,
                SeqAccum::new_stack_max(),
            ) {
                *dps_normal += accum.get_per_second();
                *volley_normal += accum.instances.max;
            }
        }
        StatTimeOptions::Sim(sim_options) => match sim_options.time.map(|v| v.into_pvalue()) {
            Some(time) if time > PValue::ZERO => {
                if let Some(accum) = aggr_proj_time(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    ospec,
                    (),
                    crit_options,
                    projectee_uid,
                    SeqAccum::new_stack_max(),
                    time,
                ) {
                    *dps_normal += accum.get_per_second();
                    *volley_normal += accum.instances.max;
                }
            }
            _ => {
                let accums = aggr_proj_split(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    ospec,
                    (),
                    crit_options,
                    projectee_uid,
                    SeqAccum::new_stack_max(),
                    SeqInstanceAccumMax::new(),
                );
                if let Some(accum_dps) = accums.get_per_second() {
                    *dps_normal += accum_dps;
                }
                if let Some(aaccum_volley) = accums.get_max() {
                    *volley_normal += aaccum_volley;
                }
            }
        },
    }
}
