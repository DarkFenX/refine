use crate::{
    def::{AttrVal, OF},
    rd::{REffectKey, REffectLocalOpcSpec, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_local_clip, aggr_proj_clip},
        calc::Calc,
        cycle::{CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatTank, Vast, vaste_stats::item_checks::check_drone_fighter_ship},
    },
    ud::{UItem, UItemKey},
    util::{RMapRMap, RMapRMapRMap},
};

pub struct StatLayerHp {
    pub buffer: AttrVal,
    pub ancil_local: AttrVal,
    pub ancil_remote: AttrVal,
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_hp(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatTank<StatLayerHp>, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_key)?;
        Ok(self.get_stat_item_hp_unchecked(ctx, calc, item_key, item))
    }
    pub(super) fn get_stat_item_hp_unchecked(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        item: &UItem,
    ) -> StatTank<StatLayerHp> {
        let attr_consts = ctx.ac();
        // Buffer - if item is not loaded, fetching those will fail
        let shield_buffer = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.shield_capacity, OF(0.0))
            .unwrap();
        let armor_buffer = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.armor_hp, OF(0.0))
            .unwrap();
        let hull_buffer = calc
            .get_item_oattr_afb_oextra(ctx, item_key, attr_consts.hp, OF(0.0))
            .unwrap();
        // Local ancillary repairs
        let (local_asb, local_aar) = match item {
            UItem::Ship(u_ship) => {
                let fit_data = self.get_fit_data(&u_ship.get_fit_key());
                let local_asb = get_local_ancil_hp(ctx, calc, &fit_data.lr_shield_limitable);
                let local_aar = get_local_ancil_hp(ctx, calc, &fit_data.lr_armor_limitable);
                (local_asb, local_aar)
            }
            _ => (OF(0.0), OF(0.0)),
        };
        // Incoming remote ancillary repairs
        let remote_asb = get_remote_ancil_hp(ctx, calc, item_key, &self.irr_shield_limitable);
        let remote_aar = get_remote_ancil_hp(ctx, calc, item_key, &self.irr_armor_limitable);
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
                ancil_local: OF(0.0),
                ancil_remote: OF(0.0),
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
    ancil_data: &RMapRMap<UItemKey, REffectKey, REffectLocalOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut total_ancil_hp = OF(0.0);
    for (&item_key, item_data) in ancil_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, ANCIL_CYCLE_OPTIONS, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            match aggr_local_clip(ctx, calc, item_key, effect, cseq, ospec) {
                Some(effect_clip_data) => total_ancil_hp += effect_clip_data.amount,
                None => continue,
            };
        }
    }
    total_ancil_hp
}

fn get_remote_ancil_hp(
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_key: UItemKey,
    ancil_data: &RMapRMapRMap<UItemKey, UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut total_ancil_hp = OF(0.0);
    let incoming_ancils = match ancil_data.get_l1(&projectee_item_key) {
        Some(incoming_ancils) => incoming_ancils,
        None => return total_ancil_hp,
    };
    for (&projector_item_key, projector_data) in incoming_ancils.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, projector_item_key, ANCIL_CYCLE_OPTIONS, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_key, ospec) in projector_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            match aggr_proj_clip(
                ctx,
                calc,
                projector_item_key,
                effect,
                cseq,
                ospec,
                Some(projectee_item_key),
            ) {
                Some(effect_clip_data) => total_ancil_hp += effect_clip_data.amount,
                None => continue,
            };
        }
    }
    total_ancil_hp
}
