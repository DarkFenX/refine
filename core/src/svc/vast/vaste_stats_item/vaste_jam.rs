use super::checks::check_item_key_drone_fighter_ship;
use crate::{
    def::OF,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        err::StatItemCheckError,
        vast::{StatJamApplied, StatSensorKind, Vast},
    },
    ud::UItemKey,
};

const JAM_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Sim,
    reload_optionals: false,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_incoming_jam(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatJamApplied, StatItemCheckError> {
        check_item_key_drone_fighter_ship(ctx, item_key)?;
        Ok(self.internal_get_stat_item_incoming_jam_unchecked(ctx, calc, item_key))
    }
    fn internal_get_stat_item_incoming_jam_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        projectee_item_key: UItemKey,
    ) -> StatJamApplied {
        let incoming_ecms = match self.in_ecm.get_l1(&projectee_item_key) {
            Some(incoming_ecms) => incoming_ecms,
            None => {
                return StatJamApplied {
                    chance: OF(0.0),
                    uptime: OF(0.0),
                };
            }
        };
        let sensor = Vast::internal_get_stat_item_sensor_unchecked(ctx, calc, projectee_item_key);
        let mut item_unjam_chance = OF(1.0);
        let mut item_unjam_uptime = OF(1.0);
        for (&projector_item_key, projector_data) in incoming_ecms.iter() {
            for (&effect_key, ecm_getter) in projector_data.iter() {
                let effect = ctx.u_data.src.get_effect(effect_key);
                let item_ecm = match ecm_getter(ctx, calc, projector_item_key, effect, Some(projectee_item_key)) {
                    Some(item_ecm) => item_ecm,
                    None => continue,
                };
                let item_ecm_str = match sensor.kind {
                    StatSensorKind::Radar => item_ecm.radar,
                    StatSensorKind::Magnetometric => item_ecm.magnetometric,
                    StatSensorKind::Gravimetric => item_ecm.gravimetric,
                    StatSensorKind::Ladar => item_ecm.ladar,
                };
                if item_ecm_str <= OF(0.0) {
                    continue;
                }
                // Jam chance
                let ecm_jam_chance = (item_ecm_str / sensor.strength).clamp(OF(0.0), OF(1.0));
                item_unjam_chance *= OF(1.0) - ecm_jam_chance;
                // Jam uptime
                if let Some(cycle_map) = get_item_cycle_info(ctx, calc, projector_item_key, JAM_OPTIONS, false)
                    && let Some(effect_cycle) = cycle_map.get(&effect_key)
                    && effect_cycle.is_infinite()
                {
                    // Theoretically, it is possible to have overlapping cycles with some items
                    // (e.g. if ECM burst projectors had super short cycle). This stat deliberately
                    // gives up on that by restricting cycle uptime to 100% because:
                    // - EVE has no such known scenarios;
                    // - handling it properly would be much more complex;
                    // - uptime is an approximate stat by its nature (since value depends on how item cycles would be
                    //   distributed, and the lib does not expose controls to that).
                    let cycle_uptime =
                        (item_ecm.duration / effect_cycle.get_average_cycle_time()).clamp(OF(0.0), OF(1.0));
                    item_unjam_uptime *= OF(1.0) - ecm_jam_chance * cycle_uptime;
                }
            }
        }
        StatJamApplied {
            chance: OF(1.0) - item_unjam_chance,
            uptime: OF(1.0) - item_unjam_uptime,
        }
    }
}
