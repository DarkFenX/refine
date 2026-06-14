use crate::{
    misc::{InfCount, ItemSpoolInfo, OptionalReload},
    num::Count,
    svc::{
        Svc, SvcCtx,
        cycle::{CseqMap, CycleOptionsSim, CyclingOptions, get_item_cseq_map},
        spool::ResolvedSpool,
    },
    ud::{UData, UItemId},
};

const CYCLE_COUNT_OPTIONS: CyclingOptions = CyclingOptions::Sim(CycleOptionsSim {
    optional_reloads: Some(OptionalReload::OnEmpty),
    ..
});

impl Svc {
    pub(crate) fn get_item_charged_cycle_count(
        &mut self,
        reuse_cseq_map: &mut CseqMap,
        u_data: &UData,
        item_uid: UItemId,
    ) -> Option<InfCount> {
        let u_item = u_data.items.get(item_uid);
        let defeff_rid = u_item.get_defeff_rid()??;
        if !get_item_cseq_map(
            reuse_cseq_map,
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_uid,
            CYCLE_COUNT_OPTIONS,
        ) {
            return None;
        }
        let mut charged_cycles = Count::ZERO;
        let cseq = reuse_cseq_map.get(&defeff_rid)?;
        let cycle_parts = cseq.get_parts();
        for cycle_part in cycle_parts.iter() {
            // Current part uncharged means item is empty by this point
            if cycle_part.data.active.chargedness.is_none() {
                return Some(InfCount::Count(charged_cycles));
            }
            let repeat_count = match cycle_part.repeat_count {
                InfCount::Count(repeat_count) => repeat_count,
                // If some of the parts are charged, and they cycle infinitely, item never goes
                // "empty"
                InfCount::Infinite => return Some(InfCount::Infinite),
            };
            charged_cycles += repeat_count;
            // Break sequence only on reloads
            if let Some(soft_dt) = cycle_part.data.soft_dt
                && soft_dt.reasons.reload
            {
                return Some(InfCount::Count(charged_cycles));
            }
        }
        // If we:
        // - didn't bail early
        // - have charged cycles
        // - end of sequence has no hard downtime
        // - sequence is looped
        // Then it is a never-ending sequence of charged cycles
        if cycle_parts.loops && charged_cycles > Count::ZERO && cseq.get_hard_dt().is_none() {
            return Some(InfCount::Infinite);
        }
        Some(InfCount::Count(charged_cycles))
    }
    pub(crate) fn get_effect_spool_cycle_count(&mut self, u_data: &UData, item_uid: UItemId) -> Option<ItemSpoolInfo> {
        let u_item = u_data.items.get(item_uid);
        let defeff_rid = u_item.get_defeff_rid()??;
        let defeff = u_data.src.get_effect_by_rid(defeff_rid);
        let spool_attrs = defeff.spool_attr_rids?;
        // TODO: limit by non-interrupted spool cycle count
        let ctx = SvcCtx::new(u_data, &self.eff_projs);
        let resolved_spool = ResolvedSpool::try_build(ctx, &mut self.calc, item_uid, defeff, None, spool_attrs)?;
        let overridden = u_item.get_spool().is_some();
        Some(ItemSpoolInfo {
            current: resolved_spool.cycles,
            max: resolved_spool.cycles_max,
            overridden,
        })
    }
}
