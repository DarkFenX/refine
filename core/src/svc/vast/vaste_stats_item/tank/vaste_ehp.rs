use super::{super::checks::check_drone_fighter_ship, shared::get_tanking_efficiency};
use crate::{
    def::{AttrVal, OF},
    misc::{DmgKinds, DpsProfile},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{StatLayerHp, StatTank, Vast},
    },
    ud::UItemKey,
};

pub struct StatLayerEhp {
    pub buffer: AttrVal,
    pub ancil_local: AttrVal,
    pub ancil_remote: AttrVal,
    pub mult: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_ehp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_key)?;
        let hp = self.get_stat_item_hp_unchecked(ctx, calc, item_key, item);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_key);
        let incoming_dps = incoming_dps.unwrap_or(ctx.u_data.default_incoming_dps);
        let shield_mult = get_tanking_efficiency(&resists.shield, incoming_dps);
        let armor_mult = get_tanking_efficiency(&resists.armor, incoming_dps);
        let hull_mult = get_tanking_efficiency(&resists.hull, incoming_dps);
        let ehp = make_ehp(hp, shield_mult, armor_mult, hull_mult);
        Ok(ehp)
    }
    pub(in crate::svc) fn get_stat_item_wc_ehp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_key)?;
        let hp = self.get_stat_item_hp_unchecked(ctx, calc, item_key, item);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_key);
        let shield_mult = get_worst_case_tanking_efficiency(&resists.shield);
        let armor_mult = get_worst_case_tanking_efficiency(&resists.armor);
        let hull_mult = get_worst_case_tanking_efficiency(&resists.hull);
        let wc_ehp = make_ehp(hp, shield_mult, armor_mult, hull_mult);
        Ok(wc_ehp)
    }
}

fn get_worst_case_tanking_efficiency(resists: &DmgKinds<AttrVal>) -> Option<AttrVal> {
    let dealt = OF(1.0);
    let absorbed = resists.iter().copied().min().unwrap();
    let received = dealt - absorbed;
    match received > OF(0.0) {
        true => Some(dealt / received),
        false => None,
    }
}

fn make_ehp(
    hp: StatTank<StatLayerHp>,
    shield_mult: Option<AttrVal>,
    armor_mult: Option<AttrVal>,
    hull_mult: Option<AttrVal>,
) -> StatTank<Option<StatLayerEhp>> {
    StatTank {
        shield: shield_mult.map(|mult| StatLayerEhp {
            buffer: hp.shield.buffer * mult,
            ancil_local: hp.shield.ancil_local * mult,
            ancil_remote: hp.shield.ancil_remote * mult,
            mult,
        }),
        armor: armor_mult.map(|mult| StatLayerEhp {
            buffer: hp.armor.buffer * mult,
            ancil_local: hp.armor.ancil_local * mult,
            ancil_remote: hp.armor.ancil_remote * mult,
            mult,
        }),
        hull: hull_mult.map(|mult| StatLayerEhp {
            buffer: hp.hull.buffer * mult,
            ancil_local: hp.hull.ancil_local * mult,
            ancil_remote: hp.hull.ancil_remote * mult,
            mult,
        }),
    }
}
