use std::collections::BinaryHeap;

use super::{
    merge::Merger,
    stagger::{StaggerKey, StatCapSimStaggerInt, process_staggers},
};
use crate::{
    misc::OptionalReload,
    num::PValue,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        vast::{
            Vast, VastFitData,
            aggr::{aggr_local_iter, aggr_proj_iter},
            stats::cap::sim::{
                event::{CapSimEvent, CapSimEventInjector},
                shared::Direction,
            },
        },
    },
    ud::UItemId,
    util::{PrefetchPeekable, RMapVec},
};

pub(in crate::svc::vast::stats::cap::sim) fn prepare_events(
    ctx: SvcCtx,
    calc: &mut Calc,
    vast: &Vast,
    optional_reloads: Option<OptionalReload>,
    stagger: StatCapSimStaggerInt,
    fit_data: &VastFitData,
    cap_item_uid: UItemId,
) -> BinaryHeap<CapSimEvent> {
    let cycling_options = CyclingOptions::Sim(CycleOptionsSim { optional_reloads, .. });
    let mut merger = Merger::new();
    fill_consumers(ctx, calc, &mut merger, cycling_options, &stagger, fit_data);
    fill_nosfs(ctx, calc, &mut merger, cycling_options, &stagger, fit_data);
    fill_incoming_neuts(ctx, calc, &mut merger, cycling_options, &stagger, vast, cap_item_uid);
    fill_incoming_transfers(ctx, calc, &mut merger, cycling_options, &stagger, vast, cap_item_uid);
    let mut events = BinaryHeap::new();
    merger.into_sim_events(&mut events);
    fill_injectors(ctx, calc, &mut events, cycling_options, fit_data);
    events
}

fn fill_consumers(
    ctx: SvcCtx,
    calc: &mut Calc,
    merger: &mut Merger,
    cycling_options: CyclingOptions,
    stagger: &StatCapSimStaggerInt,
    fit_data: &VastFitData,
) {
    let direction = Direction::Loss;
    let mut stagger_map = RMapVec::new();
    for (&item_uid, item_data) in fit_data.cap_consumers.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let iter_data = match aggr_local_iter(ctx, calc, item_uid, effect, cseq, ospec, ()) {
                Some(iter_data) => iter_data,
                None => continue,
            };
            match stagger.is_staggered(item_uid) {
                true => stagger_map.add_entry(StaggerKey::new(&iter_data), iter_data),
                false => merger.add_entry(PValue::ZERO, iter_data, direction),
            }
        }
    }
    process_staggers(stagger_map, merger, direction);
}

fn fill_nosfs(
    ctx: SvcCtx,
    calc: &mut Calc,
    merger: &mut Merger,
    cycling_options: CyclingOptions,
    stagger: &StatCapSimStaggerInt,
    fit_data: &VastFitData,
) {
    let direction = Direction::Gain;
    let mut stagger_map = RMapVec::new();
    for (&nosf_item_uid, item_data) in fit_data.cap_nosfs.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, nosf_item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let iter_data = match aggr_proj_iter(ctx, calc, nosf_item_uid, effect, cseq, ospec, (), None) {
                Some(iter_data) => iter_data,
                None => continue,
            };
            match stagger.is_staggered(nosf_item_uid) {
                true => stagger_map.add_entry(StaggerKey::new(&iter_data), iter_data),
                false => merger.add_entry(PValue::ZERO, iter_data, direction),
            }
        }
    }
    process_staggers(stagger_map, merger, direction);
}

fn fill_incoming_neuts(
    ctx: SvcCtx,
    calc: &mut Calc,
    merger: &mut Merger,
    cycling_options: CyclingOptions,
    stagger: &StatCapSimStaggerInt,
    vast: &Vast,
    cap_item_uid: UItemId,
) {
    let neut_data = match vast.in_neuts.get_l1(&cap_item_uid) {
        Some(neut_data) => neut_data,
        None => return,
    };
    let direction = Direction::Loss;
    let mut stagger_map = RMapVec::new();
    for (&neut_item_uid, item_data) in neut_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, neut_item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let iter_data = match aggr_proj_iter(ctx, calc, neut_item_uid, effect, cseq, ospec, (), Some(cap_item_uid))
            {
                Some(iter_data) => iter_data,
                None => continue,
            };
            match stagger.is_staggered(neut_item_uid) {
                true => stagger_map.add_entry(StaggerKey::new(&iter_data), iter_data),
                false => merger.add_entry(PValue::ZERO, iter_data, direction),
            }
        }
    }
    process_staggers(stagger_map, merger, direction);
}

fn fill_incoming_transfers(
    ctx: SvcCtx,
    calc: &mut Calc,
    merger: &mut Merger,
    cycling_options: CyclingOptions,
    stagger: &StatCapSimStaggerInt,
    vast: &Vast,
    cap_item_uid: UItemId,
) {
    let transfer_data = match vast.in_cap.get_l1(&cap_item_uid) {
        Some(transfer_data) => transfer_data,
        None => return,
    };
    let direction = Direction::Gain;
    let mut stagger_map = RMapVec::new();
    for (&transfer_item_uid, item_data) in transfer_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, transfer_item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let iter_data = match aggr_proj_iter(
                ctx,
                calc,
                transfer_item_uid,
                effect,
                cseq,
                ospec,
                (),
                Some(cap_item_uid),
            ) {
                Some(iter_data) => iter_data,
                None => continue,
            };
            match stagger.is_staggered(transfer_item_uid) {
                true => stagger_map.add_entry(StaggerKey::new(&iter_data), iter_data),
                false => merger.add_entry(PValue::ZERO, iter_data, direction),
            }
        }
    }
    process_staggers(stagger_map, merger, direction);
}

fn fill_injectors(
    ctx: SvcCtx,
    calc: &mut Calc,
    events: &mut BinaryHeap<CapSimEvent>,
    cycling_options: CyclingOptions,
    fit_data: &VastFitData,
) {
    for (&item_uid, item_data) in fit_data.cap_injects.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        for (&effect_rid, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_rid) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
            let iter_data = match aggr_local_iter(ctx, calc, item_uid, effect, cseq, ospec, ()) {
                Some(iter_data) => iter_data,
                None => continue,
            };
            events.push(CapSimEvent::InjectorReady(CapSimEventInjector {
                time: PValue::ZERO,
                cycle_iter: PrefetchPeekable::new(iter_data.iter()),
            }));
        }
    }
}
