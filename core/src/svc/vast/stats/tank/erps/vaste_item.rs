use super::stat::{StatErps, StatErpsLayer, StatErpsLayerRegen};
use crate::{
    misc::DpsProfile,
    num::UnitInterval,
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{
            StatTimeOptions, Vast,
            stats::{item_checks::check_drone_fighter_ship, tank::shared::get_tanking_efficiency},
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_erps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        incoming_dps: Option<DpsProfile>,
        time_options: StatTimeOptions,
        shield_perc: UnitInterval,
    ) -> Result<StatErps, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_uid)?;
        let rps = self.get_stat_item_rps_unchecked(ctx, calc, item_uid, item, time_options, shield_perc);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_uid);
        let incoming_dps = incoming_dps.unwrap_or(ctx.u_data.default_incoming_dps);
        let shield_mult = get_tanking_efficiency(resists.shield, incoming_dps);
        let armor_mult = get_tanking_efficiency(resists.armor, incoming_dps);
        let hull_mult = get_tanking_efficiency(resists.hull, incoming_dps);
        let erps = StatErps {
            shield: shield_mult.map(|mult| StatErpsLayerRegen {
                local: rps.shield.local * mult,
                remote: rps.shield.remote * mult,
                remote_penalized: rps.shield.remote_penalized * mult,
                regen: rps.shield.regen * mult,
                mult,
            }),
            armor: armor_mult.map(|mult| StatErpsLayer {
                local: rps.armor.local * mult,
                remote: rps.armor.remote * mult,
                remote_penalized: rps.armor.remote_penalized * mult,
                mult,
            }),
            hull: hull_mult.map(|mult| StatErpsLayer {
                local: rps.hull.local * mult,
                remote: rps.hull.remote * mult,
                remote_penalized: rps.hull.remote_penalized * mult,
                mult,
            }),
        };
        Ok(erps)
    }
}
