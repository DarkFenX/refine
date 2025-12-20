use crate::{
    api::AdjustableCount,
    svc::{
        Svc, SvcCtx,
        cycle::{CycleOptions, CycleOptionsSim, get_item_cycle_info},
    },
    ud::{UData, UItemKey},
    util::InfCount,
};

const CYCLE_COUNT_OPTIONS: CycleOptions = CycleOptions::Sim(CycleOptionsSim {
    reload_optionals: Some(true),
    ..
});

impl Svc {
    pub(crate) fn get_item_cycles_until_empty(&mut self, u_data: &UData, item_key: UItemKey) -> Option<InfCount> {
        let u_item = u_data.items.get(item_key);
        let defeff_key = u_item.get_defeff_key()??;
        let cycle_info = get_item_cycle_info(
            SvcCtx::new(u_data, &self.eff_projs),
            &mut self.calc,
            item_key,
            CYCLE_COUNT_OPTIONS,
            true,
        )?;
        Some(cycle_info.get(&defeff_key)?.get_charged_cycles())
    }
    pub(crate) fn get_effect_spool_cycle_count(
        &mut self,
        u_data: &UData,
        item_key: UItemKey,
    ) -> Option<AdjustableCount> {
        let u_item = u_data.items.get(item_key);
        let defeff_key = u_item.get_defeff_key()??;
        let defeff = u_data.src.get_effect(defeff_key);
        let spool_resolver = defeff.spool_resolver?;
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
