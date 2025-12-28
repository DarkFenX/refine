use super::item_checks::check_drone_fighter_ship;
use crate::{
    def::OF,
    svc::{
        SvcCtx,
        aggr::aggr_proj_first_output,
        calc::Calc,
        cycle::{CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatJamApplied, StatSensorsKind, Vast},
    },
    ud::UItemKey,
};

const JAM_OPTIONS: CyclingOptions = CyclingOptions::Sim(CycleOptionsSim { .. });

impl Vast {
    pub(in crate::svc) fn get_stat_item_incoming_jam(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projectee_item_key: UItemKey,
    ) -> Result<StatJamApplied, StatItemCheckError> {
        check_drone_fighter_ship(ctx.u_data, projectee_item_key)?;
        let incoming_ecms = match self.in_ecm.get_l1(&projectee_item_key) {
            Some(incoming_ecms) => incoming_ecms,
            None => {
                return Ok(StatJamApplied {
                    chance: OF(0.0),
                    uptime: OF(0.0),
                });
            }
        };
        let sensors = Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, projectee_item_key);
        let mut projectee_unjam_chance = OF(1.0);
        let mut projectee_unjam_uptime = OF(1.0);
        for (&projector_item_key, projector_data) in incoming_ecms.iter() {
            let cseq_map = match get_item_cseq_map(ctx, calc, projector_item_key, JAM_OPTIONS, false) {
                Some(cseq_map) => cseq_map,
                None => continue,
            };
            for (&effect_key, ospec) in projector_data.iter() {
                let cseq = match cseq_map.get(&effect_key) {
                    Some(cseq) => cseq,
                    None => continue,
                };
                let effect = ctx.u_data.src.get_effect(effect_key);
                let projector_ecm_output = match aggr_proj_first_output(
                    ctx,
                    calc,
                    projector_item_key,
                    effect,
                    cseq,
                    ospec,
                    Some(projectee_item_key),
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
                if projector_ecm_str <= OF(0.0) {
                    continue;
                }
                // Jam chance
                let jam_chance = (projector_ecm_str / sensors.strength).clamp(OF(0.0), OF(1.0));
                projectee_unjam_chance *= OF(1.0) - jam_chance;
                // Jam uptime
                // Theoretically, it is possible to have overlapping cycles with some items (e.g. if
                // ECM burst projectors had super short cycle). This stat deliberately gives up on
                // that by restricting cycle uptime to 100% because:
                // - EVE has no such known scenarios;
                // - handling it properly would be much more complex;
                // - uptime is an approximate stat by its nature (since value depends on how item cycles would be
                //   distributed, and the lib does not expose controls to that).
                let cycle_uptime = (projector_ecm.duration / projector_ecm_output.time).clamp(OF(0.0), OF(1.0));
                projectee_unjam_uptime *= OF(1.0) - jam_chance * cycle_uptime;
            }
        }
        let jam = StatJamApplied {
            chance: OF(1.0) - projectee_unjam_chance,
            uptime: OF(1.0) - projectee_unjam_uptime,
        };
        Ok(jam)
    }
}
