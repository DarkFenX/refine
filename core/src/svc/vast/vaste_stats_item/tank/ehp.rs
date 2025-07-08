use crate::{
    def::{AttrVal, ItemKey, OF},
    misc::{DmgKinds, DpsProfile},
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{StatLayerHp, StatTank, Vast},
    },
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
        item_key: ItemKey,
        incoming_dps: Option<&DpsProfile>,
    ) -> Option<StatTank<StatLayerEhp>> {
        let hp = self.get_stat_item_hp(ctx, calc, item_key)?;
        let resists = Vast::get_stat_item_resists(ctx, calc, item_key)?;
        let incoming_dps = incoming_dps.unwrap_or_else(|| &ctx.uad.default_incoming_dps);
        let shield_mult = Vast::get_tanking_efficiency(&resists.shield, incoming_dps);
        let armor_mult = Vast::get_tanking_efficiency(&resists.armor, incoming_dps);
        let struct_mult = Vast::get_tanking_efficiency(&resists.structure, incoming_dps);
        Some(make_ehp(hp, shield_mult, armor_mult, struct_mult))
    }
    pub(in crate::svc) fn get_stat_item_wcehp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: ItemKey,
    ) -> Option<StatTank<StatLayerEhp>> {
        let hp = self.get_stat_item_hp(ctx, calc, item_key)?;
        let resists = Vast::get_stat_item_resists(ctx, calc, item_key)?;
        let shield_mult = Vast::get_worst_case_tanking_efficiency(&resists.shield);
        let armor_mult = Vast::get_worst_case_tanking_efficiency(&resists.armor);
        let struct_mult = Vast::get_worst_case_tanking_efficiency(&resists.structure);
        Some(make_ehp(hp, shield_mult, armor_mult, struct_mult))
    }
    fn get_tanking_efficiency(resists: &DmgKinds<AttrVal>, incoming_dps: &DpsProfile) -> AttrVal {
        let dealt = incoming_dps.get_sum_regular();
        let absorbed = incoming_dps.get_em() * resists.em
            + incoming_dps.get_thermal() * resists.thermal
            + incoming_dps.get_kinetic() * resists.kinetic
            + incoming_dps.get_explosive() * resists.explosive;
        let received = dealt - absorbed;
        dealt / received
    }
    fn get_worst_case_tanking_efficiency(resists: &DmgKinds<AttrVal>) -> AttrVal {
        let resist = resists.iter().copied().min().unwrap();
        OF(1.0) / (OF(1.0) - resist)
    }
}

fn make_ehp(
    hp: StatTank<StatLayerHp>,
    shield_mult: AttrVal,
    armor_mult: AttrVal,
    struct_mult: AttrVal,
) -> StatTank<StatLayerEhp> {
    StatTank {
        shield: StatLayerEhp {
            buffer: hp.shield.buffer * shield_mult,
            ancil_local: hp.shield.ancil_local * shield_mult,
            ancil_remote: hp.shield.ancil_remote * shield_mult,
            mult: shield_mult,
        },
        armor: StatLayerEhp {
            buffer: hp.armor.buffer * armor_mult,
            ancil_local: hp.armor.ancil_local * armor_mult,
            ancil_remote: hp.armor.ancil_remote * armor_mult,
            mult: armor_mult,
        },
        structure: StatLayerEhp {
            buffer: hp.structure.buffer * struct_mult,
            ancil_local: hp.structure.ancil_local * struct_mult,
            ancil_remote: hp.structure.ancil_remote * struct_mult,
            mult: struct_mult,
        },
    }
}
