use super::stat::{StatHp, StatHpLayer};
use crate::{
    misc::OptionalReload,
    nd::NEffectGeneralOutputGetter,
    num::{PValue, Value},
    rd::{REffectId, REffectLocalOpcSpec, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            Vast,
            aggr::{SeqAccum, aggr_local_clip, aggr_proj_clip},
            stats::item_checks::check_drone_fighter_ship,
        },
    },
    ud::{UItem, UItemId},
    util::{RMapRMap, RMapRMapRMap},
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_hp(
        &self,
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
    ) -> Result<StatHp, StatItemCheckError> {
        let item = check_drone_fighter_ship(ctx.u_data, item_uid)?;
        Ok(self.get_stat_item_hp_unchecked(reuse_cseq_map, ctx, calc, item_uid, item))
    }
    pub(in crate::svc::vast::stats::tank) fn get_stat_item_hp_unchecked(
        &self,
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        item: &UItem,
    ) -> StatHp {
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
                let fit_data = self.get_fit_data(u_ship.get_fit_uid());
                let local_asb = get_local_ancil_hp(reuse_cseq_map, ctx, calc, &fit_data.lr_shield_limitable);
                let local_aar = get_local_ancil_hp(reuse_cseq_map, ctx, calc, &fit_data.lr_armor_limitable);
                (local_asb, local_aar)
            }
            _ => (PValue::ZERO, PValue::ZERO),
        };
        // Incoming remote ancillary repairs
        let remote_asb = get_remote_ancil_hp(reuse_cseq_map, ctx, calc, item_uid, &self.irr_shield_limitable);
        let remote_aar = get_remote_ancil_hp(reuse_cseq_map, ctx, calc, item_uid, &self.irr_armor_limitable);
        StatHp {
            shield: StatHpLayer {
                buffer: shield_buffer,
                ancil_local: local_asb,
                ancil_remote: remote_asb,
            },
            armor: StatHpLayer {
                buffer: armor_buffer,
                ancil_local: local_aar,
                ancil_remote: remote_aar,
            },
            hull: StatHpLayer {
                buffer: hull_buffer,
                ancil_local: PValue::ZERO,
                ancil_remote: PValue::ZERO,
            },
        }
    }
}

const ANCIL_CYCLE_OPTIONS: CyclingOptions = CyclingOptions::Sim(CycleOptionsSim {
    optional_reloads: Some(OptionalReload::OnEmpty),
    ..
});

fn get_local_ancil_hp(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    ancil_data: &RMapRMap<UItemId, REffectId, REffectLocalOpcSpec<NEffectGeneralOutputGetter>>,
) -> PValue {
    let mut total_ancil_hp = PValue::ZERO;
    for (&item_uid, item_data) in ancil_data.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, ANCIL_CYCLE_OPTIONS) {
            continue;
        }
        for (&effect_rid, ospec) in item_data.iter() {
            let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                continue;
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(accum) = aggr_local_clip(ctx, calc, item_uid, effect, cseq, ospec, (), SeqAccum::new_stack()) {
                total_ancil_hp += accum.instances.stacked;
            }
        }
    }
    total_ancil_hp
}

fn get_remote_ancil_hp(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    projectee_item_uid: UItemId,
    ancil_data: &RMapRMapRMap<UItemId, UItemId, REffectId, REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
) -> PValue {
    let mut total_ancil_hp = PValue::ZERO;
    let Some(incoming_ancils) = ancil_data.get_l1(&projectee_item_uid) else {
        return total_ancil_hp;
    };
    for (&projector_item_uid, projector_data) in incoming_ancils.iter() {
        if !get_item_cseq_map(reuse_cseq_map, ctx, calc, projector_item_uid, ANCIL_CYCLE_OPTIONS) {
            continue;
        }
        for (&effect_rid, ospec) in projector_data.iter() {
            let Some(cseq) = reuse_cseq_map.get(&effect_rid) else {
                continue;
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            if let Some(accum) = aggr_proj_clip(
                ctx,
                calc,
                projector_item_uid,
                effect,
                cseq,
                ospec,
                (),
                Some(projectee_item_uid),
                SeqAccum::new_stack(),
            ) {
                total_ancil_hp += accum.instances.stacked;
            }
        }
    }
    total_ancil_hp
}
