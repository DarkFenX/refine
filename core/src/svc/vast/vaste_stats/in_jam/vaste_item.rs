use super::stat::StatInJam;
use crate::{
    num::{PValue, UnitInterval, Value},
    svc::{
        SvcCtx,
        aggr::aggr_proj_first_output,
        calc::Calc,
        cycle::{CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatSensorsKind, Vast, vaste_stats::item_checks::check_drone_fighter_ship},
    },
    ud::UItemId,
};

const JAM_OPTIONS: CyclingOptions = CyclingOptions::Sim(CycleOptionsSim { .. });

impl Vast {
    pub(in crate::svc) fn get_stat_item_incoming_jam(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projectee_item_uid: UItemId,
    ) -> Result<StatInJam, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, projectee_item_uid)?;
        let incoming_ecms = match self.in_ecm.get_l1(&projectee_item_uid) {
            Some(incoming_ecms) => incoming_ecms,
            None => {
                return Ok(StatInJam {
                    chance: UnitInterval::ZERO,
                    uptime: UnitInterval::ZERO,
                });
            }
        };
        let sensors = Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, projectee_item_uid);
        let mut projectee_unjam_chance = Value::ONE;
        let mut projectee_unjam_uptime = Value::ONE;
        for (&projector_item_uid, projector_data) in incoming_ecms.iter() {
            let cseq_map = match get_item_cseq_map(ctx, calc, projector_item_uid, JAM_OPTIONS, false) {
                Some(cseq_map) => cseq_map,
                None => continue,
            };
            for (&effect_rid, ospec) in projector_data.iter() {
                let cseq = match cseq_map.get(&effect_rid) {
                    Some(cseq) => cseq,
                    None => continue,
                };
                let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
                let projector_ecm_output = match aggr_proj_first_output(
                    ctx,
                    calc,
                    projector_item_uid,
                    effect,
                    cseq,
                    ospec,
                    Some(projectee_item_uid),
                    None,
                ) {
                    Some(projector_ecm) => projector_ecm,
                    None => continue,
                };
                let projector_ecm = projector_ecm_output.output.get_amount();
                let projector_ecm_str = match sensors.kind {
                    StatSensorsKind::Radar => projector_ecm.radar,
                    StatSensorsKind::Magnetometric => projector_ecm.magnetometric,
                    StatSensorsKind::Gravimetric => projector_ecm.gravimetric,
                    StatSensorsKind::Ladar => projector_ecm.ladar,
                };
                if projector_ecm_str <= PValue::ZERO {
                    continue;
                }
                // Jam chance
                let jam_chance = UnitInterval::from_pvalue_clamped(projector_ecm_str / sensors.strength);
                projectee_unjam_chance *= Value::ONE - jam_chance.into_value();
                // Jam uptime
                // Theoretically, it is possible to have overlapping cycles with some items (e.g. if
                // ECM burst projectors had super short cycle). This stat deliberately gives up on
                // that by restricting cycle uptime to 100% because:
                // - EVE has no such known scenarios;
                // - handling it properly would be much more complex;
                // - uptime is an approximate stat by its nature (since value depends on how item cycles would be
                //   distributed, and the lib does not expose controls to that).
                let cycle_uptime =
                    UnitInterval::from_pvalue_clamped(projector_ecm.duration / projector_ecm_output.time);
                projectee_unjam_uptime *= Value::ONE - jam_chance.into_value() * cycle_uptime.into_value();
            }
        }
        let jam = StatInJam {
            chance: UnitInterval::from_value_clamped(Value::ONE - projectee_unjam_chance),
            uptime: UnitInterval::from_value_clamped(Value::ONE - projectee_unjam_uptime),
        };
        Ok(jam)
    }
}
