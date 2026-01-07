use crate::{
    api::Adjustable,
    svc::{
        Svc, SvcCtx,
        cycle::{CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        spool::ResolvedSpool,
    },
    ud::{UData, UItemId},
    util::InfCount,
};

const CYCLE_COUNT_OPTIONS: CyclingOptions = CyclingOptions::Sim(CycleOptionsSim {
    reload_optionals: Some(true),
    ..
});

impl Svc {
    pub(crate) fn get_item_cycles_until_empty(&mut self, u_data: &UData, item_key: UItemId) -> Option<InfCount> {
        let u_item = u_data.items.get(item_key);
        let defeff_key = u_item.get_defeff_rid()??;
        let cycle_info = get_item_cseq_map(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            CYCLE_COUNT_OPTIONS,
            true,
        )?;
        let mut charged_cycles = 0;
        let cycle_parts = cycle_info.get(&defeff_key)?.get_cseq_parts();
        for cycle_part in cycle_parts.iter() {
            // Current part uncharged means we're empty by this point
            if cycle_part.data.chargedness.is_none() {
                return Some(InfCount::Count(charged_cycles));
            }
            let repeat_count = match cycle_part.repeat_count {
                InfCount::Count(repeat_count) => repeat_count,
                // If some of the parts are charged, and they cycle infinitely, item never goes
                // "empty"
                InfCount::Infinite => return Some(InfCount::Infinite),
            };
            charged_cycles += repeat_count;
            // break sequence only on reloads
            if let Some(interrupt) = cycle_part.data.interrupt
                && interrupt.reload
            {
                return Some(InfCount::Count(charged_cycles));
            }
        }
        // If we didn't bail early, have charged cycles and sequence is looped, it is never-ending
        // sequence of charged cycles
        if cycle_parts.loops && charged_cycles > 0 {
            return Some(InfCount::Infinite);
        }
        Some(InfCount::Count(charged_cycles))
    }
    pub(crate) fn get_effect_spool_cycle_count(&mut self, u_data: &UData, item_key: UItemId) -> Option<Adjustable> {
        let u_item = u_data.items.get(item_key);
        let defeff_key = u_item.get_defeff_rid()??;
        let defeff = u_data.src.get_effect_by_rid(defeff_key);
        let spool_attrs = defeff.spool_attr_rids?;
        // TODO: limit by non-interrupted spool cycle count
        let ctx = SvcCtx::new(u_data, &self.eff_projs);
        let resolved_spool = ResolvedSpool::try_build(ctx, &mut self.calc, item_key, defeff, None, spool_attrs)?;
        let overridden = u_item.get_spool().is_some();
        Some(Adjustable {
            current: resolved_spool.cycles,
            max: resolved_spool.cycles_max,
            overridden,
        })
    }
}
