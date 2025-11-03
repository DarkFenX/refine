use super::{super::checks::check_item_ship, shared::CYCLE_OPTIONS_BURST, shared::CYCLE_OPTIONS_SIM};
use crate::{
    def::{AttrVal, OF},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{Cycle, get_item_cycle_info},
        err::StatItemCheckError,
        output::Output,
        vast::Vast,
    },
    ud::UItemKey,
};

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
        Ok(StatCapSimResult::Stable(OF(0.25)))
    }
}

struct CapSimIter {
    general: Vec<(Cycle, Output<AttrVal>)>,
    injectors: Vec<(Cycle, Output<AttrVal>)>,
}
impl CapSimIter {
    fn new(ctx: SvcCtx, calc: &mut Calc, vast: &Vast, cap_item_key: UItemKey) -> Self {
        let mut general = Vec::new();
        let mut injectors = Vec::new();
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
                    general.push((effect_cycles, -output_per_cycle));
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
                    general.push((effect_cycles, output_per_cycle));
                }
            }
        }
        Self { general, injectors }
    }
}
impl Iterator for CapSimIter {
    type Item = (AttrVal, AttrVal);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
