use crate::{
    misc::AdjustableCount,
    svc::{
        Svc, SvcCtx,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
    },
    ud::{UData, UItemKey},
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
    pub(crate) fn get_item_cycles_until_reload(&mut self, u_data: &UData, item_key: UItemKey) -> Option<InfCount> {
        let u_item = u_data.items.get(item_key);
        let defeff_id = u_item.get_defeff_key()??;
        let cycle_info = get_item_cycle_info(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            CUR_CYCLE_OPTIONS,
            true,
        )?;
        Some(cycle_info.get(&defeff_id)?.get_cycles_until_reload())
    }
    pub(crate) fn get_effect_spool_cycle_count(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Option<AdjustableCount> {
        let u_item = u_data.items.get(item_key);
        let defeff_key = u_item.get_defeff_key()??;
        let defeff = u_data.src.get_effect(defeff_key);
        let spool_resolver = defeff.get_spool_resolver()?;
        let resolved_spool = spool_resolver(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            defeff,
            None,
        )?;
        let overridden = u_item.get_spool().is_some();
        Some(AdjustableCount {
            current: resolved_spool.cycles,
            max: resolved_spool.cycles_max,
            overridden,
        })
    }
}
