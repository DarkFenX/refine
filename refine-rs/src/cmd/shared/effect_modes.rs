use rc::ItemMutCommon;

#[derive(Default)]
pub(in crate::cmd) struct EffectModes {
    data: Vec<(rc::EffectId, rc::EffectMode)>,
}
impl EffectModes {
    pub(in crate::cmd) fn apply(&self, core_item: &mut impl ItemMutCommon) {
        if !self.data.is_empty() {
            core_item.set_effect_modes(self.data.iter().map(|(k, v)| (*k, *v)));
        }
    }
}
