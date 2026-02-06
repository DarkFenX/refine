use super::stat::StatInJam;
use crate::{
    num::{UnitInterval, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            Vast,
            aggr::{SeqAccum, aggr_proj_first},
            stats::item_checks::check_drone_fighter_ship,
        },
    },
    ud::UItemId,
};

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
            let cseq_map = match get_item_cseq_map(ctx, calc, projector_item_uid, CyclingOptions::Burst, false) {
                Some(cseq_map) => cseq_map,
                None => continue,
            };
            for (&effect_rid, ospec) in projector_data.iter() {
                let cseq = match cseq_map.get(&effect_rid) {
                    Some(cseq) => cseq,
                    None => continue,
                };
                let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
                let mut accum = SeqAccum::new_jam_chance(sensors);
                if aggr_proj_first(
                    ctx,
                    calc,
                    projector_item_uid,
                    effect,
                    cseq,
                    ospec,
                    Some(projectee_item_uid),
                    None,
                    &mut accum,
                ) {
                    projectee_unjam_chance *= accum.get_unjam_chance().into_value();
                    projectee_unjam_uptime *= accum.get_unjam_uptime().into_value();
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
