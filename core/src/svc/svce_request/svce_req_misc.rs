use crate::{
    def::ItemKey,
    misc::{AdjustableCount, EffectSpec},
    svc::{Svc, SvcCtx, efuncs},
    uad::Uad,
    util::InfCount,
};

impl Svc {
    pub(crate) fn get_effect_cycle_count(&self, uad: &Uad, item_key: ItemKey) -> Option<InfCount> {
        let uad_item = uad.items.get(item_key);
        let defeff_id = uad_item.get_a_defeff_id()??;
        efuncs::get_espec_cycle_count(SvcCtx::new(uad, &self.eprojs), EffectSpec::new(item_key, defeff_id))
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
