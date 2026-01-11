use super::stat::{StatEhp, StatEhpLayer};
use crate::{
    misc::DpsProfile,
    num::{PValue, Value},
    svc::{
        SvcCtx,
        calc::Calc,
        err::StatItemCheckError,
        vast::{
            StatHp, StatResistsLayer, Vast,
            vaste_stats::{item_checks::check_drone_fighter_ship, tank::shared::get_tanking_efficiency},
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_ehp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        incoming_dps: Option<DpsProfile>,
    ) -> Result<StatEhp, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_uid)?;
        let hp = self.get_stat_item_hp_unchecked(ctx, calc, item_uid, item);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_uid);
        let incoming_dps = incoming_dps.unwrap_or(ctx.u_data.default_incoming_dps);
        let shield_mult = get_tanking_efficiency(resists.shield, incoming_dps);
        let armor_mult = get_tanking_efficiency(resists.armor, incoming_dps);
        let hull_mult = get_tanking_efficiency(resists.hull, incoming_dps);
        let ehp = make_ehp(hp, shield_mult, armor_mult, hull_mult);
        Ok(ehp)
    }
    pub(in crate::svc) fn get_stat_item_wc_ehp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<StatEhp, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_uid)?;
        let hp = self.get_stat_item_hp_unchecked(ctx, calc, item_uid, item);
        let resists = Vast::get_stat_item_resists_unchecked(ctx, calc, item_uid);
        let shield_mult = get_worst_case_tanking_efficiency(resists.shield);
        let armor_mult = get_worst_case_tanking_efficiency(resists.armor);
        let hull_mult = get_worst_case_tanking_efficiency(resists.hull);
        let wc_ehp = make_ehp(hp, shield_mult, armor_mult, hull_mult);
        Ok(wc_ehp)
    }
}

fn get_worst_case_tanking_efficiency(resists: StatResistsLayer) -> Option<PValue> {
    let dealt = PValue::ONE;
    let absorbed = [resists.em, resists.thermal, resists.kinetic, resists.explosive]
        .into_iter()
        .min()
        .unwrap();
    let received = dealt - absorbed.into_pvalue();
    let received = match received > Value::ZERO {
        true => PValue::from_value_unchecked(received),
        false => return None,
    };
    Some(dealt / received)
}

fn make_ehp(hp: StatHp, shield_mult: Option<PValue>, armor_mult: Option<PValue>, hull_mult: Option<PValue>) -> StatEhp {
    StatEhp {
        shield: shield_mult.map(|mult| StatEhpLayer {
            buffer: hp.shield.buffer * mult,
            ancil_local: hp.shield.ancil_local * mult,
            ancil_remote: hp.shield.ancil_remote * mult,
            mult,
        }),
        armor: armor_mult.map(|mult| StatEhpLayer {
            buffer: hp.armor.buffer * mult,
            ancil_local: hp.armor.ancil_local * mult,
            ancil_remote: hp.armor.ancil_remote * mult,
            mult,
        }),
        hull: hull_mult.map(|mult| StatEhpLayer {
            buffer: hp.hull.buffer * mult,
            ancil_local: hp.hull.ancil_local * mult,
            ancil_remote: hp.hull.ancil_remote * mult,
            mult,
        }),
    }
}
