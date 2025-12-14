use super::{super::checks::check_drone_fighter_ship, shared::get_tanking_efficiency};
use crate::{
    def::AttrVal,
    misc::{DpsProfile, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{StatTankRegen, Vast},
    },
    ud::UItemKey,
    util::UnitInterval,
};

pub struct StatLayerErps {
    pub local: AttrVal,
    pub remote: AttrVal,
    pub remote_penalized: AttrVal,
    pub mult: AttrVal,
}

pub struct StatLayerErpsRegen {
    pub local: AttrVal,
    pub remote: AttrVal,
    pub remote_penalized: AttrVal,
    pub regen: AttrVal,
    pub mult: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_erps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        incoming_dps: Option<DpsProfile>,
        shield_perc: UnitInterval,
        spool: Option<Spool>,
    ) -> Result<StatTankRegen<Option<StatLayerErps>, Option<StatLayerErpsRegen>>, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_key)?;
        let rps = self.get_stat_item_rps_unchecked(ctx, calc, item_key, item, shield_perc, spool);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_key);
        let incoming_dps = incoming_dps.unwrap_or(ctx.u_data.default_incoming_dps);
        let shield_mult = get_tanking_efficiency(&resists.shield, incoming_dps);
        let armor_mult = get_tanking_efficiency(&resists.armor, incoming_dps);
        let hull_mult = get_tanking_efficiency(&resists.hull, incoming_dps);
        let erps = StatTankRegen {
            shield: shield_mult.map(|mult| StatLayerErpsRegen {
                local: rps.shield.local * mult,
                remote: rps.shield.remote * mult,
                remote_penalized: rps.shield.remote_penalized * mult,
                regen: rps.shield.regen * mult,
                mult,
            }),
            armor: armor_mult.map(|mult| StatLayerErps {
                local: rps.armor.local * mult,
                remote: rps.armor.remote * mult,
                remote_penalized: rps.armor.remote_penalized * mult,
                mult,
            }),
            hull: hull_mult.map(|mult| StatLayerErps {
                local: rps.hull.local * mult,
                remote: rps.hull.remote * mult,
                remote_penalized: rps.hull.remote_penalized * mult,
                mult,
            }),
        };
        Ok(erps)
    }
}
