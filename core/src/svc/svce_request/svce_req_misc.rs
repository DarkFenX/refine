use crate::{
    def::ItemKey,
    misc::{CycleCount, EffectSpec},
    svc::{Svc, SvcCtx, efuncs},
    uad::Uad,
};

impl Svc {
    pub(crate) fn get_effect_cycle_count(&self, uad: &Uad, item_key: ItemKey) -> Option<CycleCount> {
        let uad_item = uad.items.get(item_key);
        let defeff_id = uad_item.get_a_defeff_id()??;
        efuncs::get_espec_cycle_count(SvcCtx::new(uad, &self.eprojs), EffectSpec::new(item_key, defeff_id))
    }
}
