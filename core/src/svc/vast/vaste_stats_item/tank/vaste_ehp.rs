use super::shared::{get_tanking_efficiency, item_check};
use crate::{
    def::{AttrVal, ItemKey, OF},
    misc::{DmgKinds, DpsProfile},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{StatLayerHp, StatTank, Vast},
    },
    uad::UadItem,
};

pub struct StatLayerEhp {
    pub buffer: AttrVal,
    pub ancil_local: AttrVal,
    pub ancil_remote: AttrVal,
    pub mult: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_ehp_checked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        let uad_item = ctx.uad.items.get(item_key);
        item_check(item_key, uad_item)?;
        Ok(self.get_stat_item_ehp_unchecked(ctx, calc, item_key, uad_item, incoming_dps))
    }
    fn get_stat_item_ehp_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        uad_item: &UadItem,
        incoming_dps: Option<DpsProfile>,
    ) -> StatTank<Option<StatLayerEhp>> {
        let hp = self.get_stat_item_hp_unchecked(ctx, calc, item_key, uad_item);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_key);
        let incoming_dps = incoming_dps.unwrap_or(ctx.uad.default_incoming_dps);
        let shield_mult = get_tanking_efficiency(&resists.shield, incoming_dps);
        let armor_mult = get_tanking_efficiency(&resists.armor, incoming_dps);
        let hull_mult = get_tanking_efficiency(&resists.hull, incoming_dps);
        make_ehp(hp, shield_mult, armor_mult, hull_mult)
    }
    pub(in crate::svc) fn get_stat_item_wc_ehp_checked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Result<StatTank<Option<StatLayerEhp>>, StatItemCheckError> {
        let uad_item = ctx.uad.items.get(item_key);
        item_check(item_key, uad_item)?;
        Ok(self.get_stat_item_wc_ehp_unchecked(ctx, calc, item_key, uad_item))
    }
    fn get_stat_item_wc_ehp_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) -> StatTank<Option<StatLayerEhp>> {
        let hp = self.get_stat_item_hp_unchecked(ctx, calc, item_key, uad_item);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_key);
        let shield_mult = Vast::get_worst_case_tanking_efficiency(&resists.shield);
        let armor_mult = Vast::get_worst_case_tanking_efficiency(&resists.armor);
        let hull_mult = Vast::get_worst_case_tanking_efficiency(&resists.hull);
        make_ehp(hp, shield_mult, armor_mult, hull_mult)
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
