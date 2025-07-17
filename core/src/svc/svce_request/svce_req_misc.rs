use crate::{
    def::ItemKey,
    misc::{AdjustableCount, EffectSpec},
    svc::{
        Svc, SvcCtx,
        misc::{CycleOptionReload, CycleOptions, get_item_cycle_info},
    },
    uad::Uad,
    util::InfCount,
};

const CUR_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    // Should return the same count of cycles until reload regardless of options, but burst is
    // easier to calculate
    reload_mode: CycleOptionReload::Burst,
    // Use this to return cycle count for modules like ancillary reps
    reload_optionals: true,
};

impl Svc {
    pub(crate) fn get_item_cycles_until_reload(&mut self, uad: &Uad, item_key: ItemKey) -> Option<InfCount> {
        let uad_item = uad.items.get(item_key);
        let defeff_id = uad_item.get_a_defeff_id()??;
        let cycle_info = get_item_cycle_info(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            item_key,
            CUR_CYCLE_OPTIONS,
            true,
        )?;
        Some(cycle_info.get(&defeff_id)?.get_cycles_until_reload())
    }
    pub(crate) fn get_effect_spool_cycle_count(&mut self, uad: &Uad, item_key: ItemKey) -> Option<AdjustableCount> {
        let uad_item = uad.items.get(item_key);
        let defeff_id = uad_item.get_a_defeff_id()??;
        let spool_resolver = uad.src.get_a_effect(&defeff_id)?.hc.get_resolved_spool?;
        let resolved_spool = spool_resolver(
            SvcCtx::new(uad, &self.eprojs),
            &mut self.calc,
            EffectSpec::new(item_key, defeff_id),
            None,
        )?;
        let overridden = uad_item.get_spool().is_some();
        Some(AdjustableCount {
            current: resolved_spool.cycles,
            max: resolved_spool.cycles_max,
            overridden,
        })
    }
}
