use super::stat::StatInJam;
use crate::{
    num::{PValue, UnitInterval, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_burst, aggr_proj_looped, aggr_proj_time},
            stats::item_checks::check_drone_fighter_ship,
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_incoming_jam(
        &self,
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        projectee_item_uid: UItemId,
        time_options: StatTimeOptions,
    ) -> Result<StatInJam, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, projectee_item_uid)?;
        let Some(incoming_ecms) = self.in_ecm.get_l1(&projectee_item_uid) else {
            return Ok(StatInJam {
                chance: UnitInterval::ZERO,
                uptime: UnitInterval::ZERO,
            });
        };
        let sensors = Self::internal_get_stat_item_sensors_unchecked(ctx, calc, projectee_item_uid);
        let cycling_options = CyclingOptions::from_time_options(time_options);
        let mut projectee_unjam_chance = Value::ONE;
        let mut projectee_unjam_uptime = Value::ONE;
        for (&projector_item_uid, projector_data) in incoming_ecms.iter() {
            if !get_item_cseq_map(reuse_cseq_map, ctx, calc, projector_item_uid, cycling_options) {
                continue;
            }
            for (&effect_rid, ospec) in projector_data.iter() {
                let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                    continue;
                };
                let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
                match time_options {
                    StatTimeOptions::Burst(burst_opts) => {
                        if let Some(accum) = aggr_proj_burst(
                            ctx,
                            calc,
                            projector_item_uid,
                            effect,
                            cseq,
                            ospec,
                            (),
                            Some(projectee_item_uid),
                            burst_opts.spool,
                            SeqAccum::new_jam_chance(sensors),
                        ) {
                            projectee_unjam_chance *= accum.get_unjam_chance().into_value();
                            projectee_unjam_uptime *= accum.get_unjam_uptime().into_value();
                        }
                    }
                    StatTimeOptions::Sim(sim_options) => match sim_options.time {
                        Some(time) if time > PValue::ZERO => {
                            if let Some(accum) = aggr_proj_time(
                                ctx,
                                calc,
                                projector_item_uid,
                                effect,
                                cseq,
                                ospec,
                                (),
                                Some(projectee_item_uid),
                                SeqAccum::new_jam_chance(sensors),
                                time,
                            ) {
                                projectee_unjam_chance *= accum.get_unjam_chance().into_value();
                                projectee_unjam_uptime *= accum.get_unjam_uptime().into_value();
                            }
                        }
                        _ => {
                            if let Some(accum) = aggr_proj_looped(
                                ctx,
                                calc,
                                projector_item_uid,
                                effect,
                                cseq,
                                ospec,
                                (),
                                Some(projectee_item_uid),
                                SeqAccum::new_jam_chance(sensors),
                            ) {
                                // For looped version, set unjam chance to 0 if it's below 1
                                if accum.get_unjam_chance() < UnitInterval::ONE {
                                    projectee_unjam_chance = Value::ZERO;
                                }
                                projectee_unjam_uptime *= accum.get_unjam_uptime().into_value();
                            }
                        }
                    },
                };
            }
        }
        let jam = StatInJam {
            chance: UnitInterval::from_value_clamped(Value::ONE - projectee_unjam_chance),
            uptime: UnitInterval::from_value_clamped(Value::ONE - projectee_unjam_uptime),
        };
        Ok(jam)
    }
}
