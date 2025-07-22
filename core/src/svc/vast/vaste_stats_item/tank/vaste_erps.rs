use super::shared::{get_tanking_efficiency, item_check};
use crate::{
    def::AttrVal,
    misc::{DpsProfile, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{StatTank, Vast},
    },
    uad::{UadItem, UadItemKey},
};

pub struct StatLayerErps {
    pub local: AttrVal,
    pub remote: AttrVal,
    pub remote_penalized: AttrVal,
    pub mult: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_erps_checked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UadItemKey,
        incoming_dps: Option<DpsProfile>,
        spool: Option<Spool>,
    ) -> Result<StatTank<Option<StatLayerErps>>, StatItemCheckError> {
        let uad_item = ctx.uad.items.get(item_key);
        item_check(item_key, uad_item)?;
        Ok(self.get_stat_item_erps_unchecked(ctx, calc, item_key, uad_item, incoming_dps, spool))
    }
    fn get_stat_item_erps_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UadItemKey,
        uad_item: &UadItem,
        incoming_dps: Option<DpsProfile>,
        spool: Option<Spool>,
    ) -> StatTank<Option<StatLayerErps>> {
        let rps = self.get_stat_item_rps_unchecked(ctx, calc, item_key, uad_item, spool);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_key);
        let incoming_dps = incoming_dps.unwrap_or(ctx.uad.default_incoming_dps);
        let shield_mult = get_tanking_efficiency(&resists.shield, incoming_dps);
        let armor_mult = get_tanking_efficiency(&resists.armor, incoming_dps);
        let hull_mult = get_tanking_efficiency(&resists.hull, incoming_dps);
        StatTank {
            shield: shield_mult.map(|mult| StatLayerErps {
                local: rps.shield.local * mult,
                remote: rps.shield.remote * mult,
                remote_penalized: rps.shield.remote_penalized * mult,
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
        }
    }
}
