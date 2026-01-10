use crate::{
    num::{PValue, Value},
    rd::{REffectId, REffectLocalOpcSpec, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_local_clip_amount, aggr_proj_clip_amount},
        calc::Calc,
        cycle::{CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatTank, Vast, vaste_stats::item_checks::check_drone_fighter_ship},
    },
    ud::{UItem, UItemId},
    util::{RMapRMap, RMapRMapRMap},
};

pub struct StatLayerHp {
    pub buffer: PValue,
    pub ancil_local: PValue,
    pub ancil_remote: PValue,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_hp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(self.get_stat_item_hp_unchecked(ctx, calc, item_uid, item))
    }
    pub(super) fn get_stat_item_hp_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        item: &UItem,
    ) -> StatTank<StatLayerHp> {
        let attr_consts = ctx.ac();
        // Buffer - if item is not loaded, fetching those will fail
        let shield_buffer = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.shield_capacity, Value::ZERO)
                .unwrap(),
        );
        let armor_buffer = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.armor_hp, Value::ZERO)
                .unwrap(),
        );
        let hull_buffer = PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, attr_consts.hp, Value::ZERO)
                .unwrap(),
        );
        // Local ancillary repairs
        let (local_asb, local_aar) = match item {
            UItem::Ship(u_ship) => {
                let fit_data = self.get_fit_data(&u_ship.get_fit_uid());
                let local_asb = get_local_ancil_hp(ctx, calc, &fit_data.lr_shield_limitable);
                let local_aar = get_local_ancil_hp(ctx, calc, &fit_data.lr_armor_limitable);
                (local_asb, local_aar)
            }
            _ => (PValue::ZERO, PValue::ZERO),
        };
        // Incoming remote ancillary repairs
        let remote_asb = get_remote_ancil_hp(ctx, calc, item_uid, &self.irr_shield_limitable);
        let remote_aar = get_remote_ancil_hp(ctx, calc, item_uid, &self.irr_armor_limitable);
        StatTank {
            shield: StatLayerHp {
                buffer: shield_buffer,
                ancil_local: local_asb,
                ancil_remote: remote_asb,
            },
            armor: StatLayerHp {
                buffer: armor_buffer,
                ancil_local: local_aar,
                ancil_remote: remote_aar,
            },
            hull: StatLayerHp {
                buffer: hull_buffer,
                ancil_local: PValue::ZERO,
                ancil_remote: PValue::ZERO,
            },
        }
    }
}

const ANCIL_CYCLE_OPTIONS: CyclingOptions = CyclingOptions::Sim(CycleOptionsSim {
    reload_optionals: Some(true),
    ..
});

fn get_local_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    ancil_data: &RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<PValue>>,
) -> PValue {
    let mut total_ancil_hp = PValue::ZERO;
    for (&item_uid, item_data) in ancil_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, ANCIL_CYCLE_OPTIONS, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(effect_clip_data) = aggr_local_clip_amount(ctx, calc, item_uid, effect, cseq, ospec) {
                total_ancil_hp += effect_clip_data.amount;
            }
        }
    }
    total_ancil_hp
}

fn get_remote_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_uid: UItemId,
    ancil_data: &RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<PValue>>,
) -> PValue {
    let mut total_ancil_hp = PValue::ZERO;
    let incoming_ancils = match ancil_data.get_l1(&projectee_item_uid) {
        Some(incoming_ancils) => incoming_ancils,
        None => return total_ancil_hp,
    };
    for (&projector_item_uid, projector_data) in incoming_ancils.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, projector_item_uid, ANCIL_CYCLE_OPTIONS, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in projector_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(effect_clip_data) = aggr_proj_clip_amount(
                ctx,
                calc,
                projector_item_uid,
                effect,
                cseq,
                ospec,
                Some(projectee_item_uid),
            ) {
                total_ancil_hp += effect_clip_data.amount;
            }
        }
    }
    total_ancil_hp
}
