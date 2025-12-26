use super::item_checks::check_drone_fighter_ship;
use crate::{
    def::OF,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptions, CycleOptionsSim, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatJamApplied, StatSensorsKind, Vast},
    },
    ud::UItemKey,
};

const JAM_OPTIONS: CycleOptions = CycleOptions::Sim(CycleOptionsSim { .. });

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
                let jam = StatJamApplied {
                    chance: OF(0.0),
                    uptime: OF(0.0),
                };
                return Ok(jam);
            }
        };
        let sensors = Vast::internal_get_stat_item_sensors_unchecked(ctx, calc, projectee_item_key);
        let mut item_unjam_chance = OF(1.0);
        let mut item_unjam_uptime = OF(1.0);
        for (&projector_item_key, projector_data) in incoming_ecms.iter() {
            for (&effect_key, ospec) in projector_data.iter() {
                let effect = ctx.u_data.src.get_effect(effect_key);
                let inv_data = ospec.make_invar_data(ctx, calc, projector_item_key, effect, Some(projectee_item_key));
                let item_ecm = match ospec.get_total(ctx, calc, projector_item_key, effect, None, None, inv_data) {
                    Some(item_ecm) => item_ecm,
                    None => continue,
                };
                let item_ecm_str = match sensors.kind {
                    StatSensorsKind::Radar => item_ecm.radar,
                    StatSensorsKind::Magnetometric => item_ecm.magnetometric,
                    StatSensorsKind::Gravimetric => item_ecm.gravimetric,
                    StatSensorsKind::Ladar => item_ecm.ladar,
                };
                if item_ecm_str <= OF(0.0) {
                    continue;
                }
                // Jam chance
                let ecm_jam_chance = (item_ecm_str / sensors.strength).clamp(OF(0.0), OF(1.0));
                item_unjam_chance *= OF(1.0) - ecm_jam_chance;
                // Jam uptime
                if let Some(cycle_map) = get_item_cycle_info(ctx, calc, projector_item_key, JAM_OPTIONS, false)
                    && let Some(effect_cycle_loop) = cycle_map.get(&effect_key).and_then(|v| v.try_get_loop())
                {
                    // Theoretically, it is possible to have overlapping cycles with some items
                    // (e.g. if ECM burst projectors had super short cycle). This stat deliberately
                    // gives up on that by restricting cycle uptime to 100% because:
                    // - EVE has no such known scenarios;
                    // - handling it properly would be much more complex;
                    // - uptime is an approximate stat by its nature (since value depends on how item cycles would be
                    //   distributed, and the lib does not expose controls to that).
                    let cycle_uptime =
                        (item_ecm.duration / effect_cycle_loop.get_average_time()).clamp(OF(0.0), OF(1.0));
                    item_unjam_uptime *= OF(1.0) - ecm_jam_chance * cycle_uptime;
                }
            }
        }
        let jam = StatJamApplied {
            chance: OF(1.0) - item_unjam_chance,
            uptime: OF(1.0) - item_unjam_uptime,
        };
        Ok(jam)
    }
}
