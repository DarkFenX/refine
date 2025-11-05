use std::{cmp::Ordering, collections::BinaryHeap};

use super::{
    super::checks::check_item_ship,
    shared::{CYCLE_OPTIONS_BURST, CYCLE_OPTIONS_SIM},
};
use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, CycleIter, get_item_cycle_info},
        err::StatItemCheckError,
        output::{Output, OutputSimple},
        vast::{Vast, VastFitData},
    },
    ud::UItemKey,
};

const TIME_LIMIT: AttrVal = OF(24.0 * 60.0 * 60.0);

pub enum StatCapSimResult {
    // Low watermark of stability value
    Stable(AttrVal),
    // Time in seconds it takes to drain cap to 0
    Unstable(AttrVal),
}

impl Vast {
    pub(in crate::svc) fn get_stat_item_cap_sim(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
    ) -> Result<StatCapSimResult, StatItemCheckError> {
        let item = ctx.u_data.items.get(item_key);
        check_item_ship(item_key, item)?;
        let fit_data = self.fit_datas.get(&item.get_ship().unwrap().get_fit_key()).unwrap();
        for (time, amount) in CapSimIter::new(ctx, calc, self, fit_data, item_key) {
            if time > TIME_LIMIT {
                break;
            }
        }
        Ok(StatCapSimResult::Stable(OF(0.25)))
    }
}

enum CapSimEvent {
    // Next event time, amount
    CapChange(AttrVal, AttrVal),
    // Next event time, iterator, output
    Cycle(AttrVal, CycleIter, Output<AttrVal>),
}
impl CapSimEvent {
    fn get_time(&self) -> AttrVal {
        match self {
            Self::CapChange(time, _) => *time,
            Self::Cycle(time, _, _) => *time,
        }
    }
    fn get_amount(&self) -> Option<AttrVal> {
        match self {
            Self::CapChange(_, amount) => Some(*amount),
            Self::Cycle(_, _, _) => None,
        }
    }
}
impl PartialOrd for CapSimEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CapSimEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Since sim is using max-heap, adjust parameters so that:
        // - events which have lower time are processed earlier
        // - with equal time, cycle events are processed first, then cap change events amount desc
        match other.get_time().cmp(&self.get_time()) {
            Ordering::Equal => match (self.get_amount(), other.get_amount()) {
                (Some(s), Some(o)) => s.cmp(&o),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            result => result,
        }
    }
}
impl PartialEq<Self> for CapSimEvent {
    fn eq(&self, other: &Self) -> bool {
        self.get_time() == other.get_time() && self.get_amount() == other.get_amount()
    }
}
impl Eq for CapSimEvent {}

struct CapSimIter {
    events: BinaryHeap<CapSimEvent>,
    injectors: Vec<(Cycle, Output<AttrVal>)>,
}
impl CapSimIter {
    fn new(ctx: SvcCtx, calc: &mut Calc, vast: &Vast, fit_data: &VastFitData, cap_item_key: UItemKey) -> Self {
        let mut injectors = Vec::new();
        let mut events = BinaryHeap::new();
        // Consumers
        for (&item_key, item_data) in fit_data.cap_consumers.iter() {
            let mut cycle_map = match get_item_cycle_info(ctx, calc, item_key, CYCLE_OPTIONS_SIM, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (&effect_key, attr_id) in item_data.iter() {
                let cap_used = match calc.get_item_attr_val_extra(ctx, item_key, attr_id) {
                    Ok(cap_used) => cap_used,
                    Err(_) => continue,
                };
                let effect_cycles = match cycle_map.remove(&effect_key) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                let output = Output::Simple(OutputSimple {
                    amount: -cap_used,
                    delay: OF(0.0),
                });
                events.push(CapSimEvent::Cycle(OF(0.0), effect_cycles.iter_cycles(), output));
            }
        }
        // Neuts
        if let Some(neut_data) = vast.in_neuts.get_l1(&cap_item_key) {
            for (&neut_item_key, item_data) in neut_data.iter() {
                let mut cycle_map = match get_item_cycle_info(ctx, calc, neut_item_key, CYCLE_OPTIONS_BURST, false) {
                    Some(cycle_map) => cycle_map,
                    None => continue,
                };
                for (&effect_key, cap_getter) in item_data.iter() {
                    let effect = ctx.u_data.src.get_effect(effect_key);
                    let output_per_cycle = match cap_getter(ctx, calc, neut_item_key, effect, Some(cap_item_key)) {
                        Some(output_per_cycle) => output_per_cycle,
                        None => continue,
                    };
                    let effect_cycles = match cycle_map.remove(&effect_key) {
                        Some(effect_cycles) => effect_cycles,
                        None => continue,
                    };
                    events.push(CapSimEvent::Cycle(
                        OF(0.0),
                        effect_cycles.iter_cycles(),
                        output_per_cycle,
                    ));
                }
            }
        };
        // Cap transfers
        if let Some(transfer_data) = vast.in_cap.get_l1(&cap_item_key) {
            for (&transfer_item_key, item_data) in transfer_data.iter() {
                let mut cycle_map = match get_item_cycle_info(ctx, calc, transfer_item_key, CYCLE_OPTIONS_BURST, false)
                {
                    Some(cycle_map) => cycle_map,
                    None => continue,
                };
                for (&effect_key, cap_getter) in item_data.iter() {
                    let effect = ctx.u_data.src.get_effect(effect_key);
                    let output_per_cycle =
                        match cap_getter(ctx, calc, transfer_item_key, effect, None, Some(cap_item_key)) {
                            Some(output_per_cycle) => output_per_cycle,
                            None => continue,
                        };
                    let effect_cycles = match cycle_map.remove(&effect_key) {
                        Some(effect_cycles) => effect_cycles,
                        None => continue,
                    };
                    events.push(CapSimEvent::Cycle(
                        OF(0.0),
                        effect_cycles.iter_cycles(),
                        output_per_cycle,
                    ));
                }
            }
        }
        // Cap injectors
        for (&item_key, item_data) in fit_data.cap_boosts.iter() {
            let mut cycle_map = match get_item_cycle_info(ctx, calc, item_key, CYCLE_OPTIONS_SIM, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (&effect_key, cap_getter) in item_data.iter() {
                let output_per_cycle = match cap_getter(ctx, calc, item_key) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                let effect_cycles = match cycle_map.remove(&effect_key) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                injectors.push((effect_cycles, output_per_cycle));
            }
        }
        Self { events, injectors }
    }
}
impl Iterator for CapSimIter {
    type Item = (AttrVal, AttrVal);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.events.pop() {
            match event {
                CapSimEvent::Cycle(event_time, mut cycle_iter, output_per_cycle) => {
                    // Add outputs for this cycle
                    for (output_delay, output_value) in output_per_cycle.iter_output() {
                        let next_event = CapSimEvent::CapChange(event_time + output_delay, output_value);
                        self.events.push(next_event);
                    }
                    // Schedule next cycle, if any
                    if let Some(next_cycle_delay) = cycle_iter.next() {
                        let next_event =
                            CapSimEvent::Cycle(event_time + next_cycle_delay, cycle_iter, output_per_cycle);
                        self.events.push(next_event);
                    }
                }
                CapSimEvent::CapChange(event_time, cap_change_amount) => return Some((event_time, cap_change_amount)),
            }
        }
        None
    }
}
