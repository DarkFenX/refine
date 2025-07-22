use crate::{
    ad,
    def::ItemId,
    misc::EffectMode,
    src::Src,
    uad::{
        UadFitKey, UadItemKey,
        item::{Projs, UadEffectUpdates, UadItemBase},
    },
    util::{Named, RMap, RSet},
};

const DISABLED_STATE: ad::AState = ad::AState::Ghost;

#[derive(Clone)]
pub(crate) struct UadCharge {
    base: UadItemBase,
    fit_key: UadFitKey,
    cont_key: UadItemKey,
    projs: Projs,
    // Stores container state when charge is force disabled
    stored_cont_a_state: Option<ad::AState>,
}
impl UadCharge {
    pub(crate) fn new(
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_key: UadFitKey,
        cont_key: UadItemKey,
        cont_a_state: ad::AState,
        force_disable: bool,
        src: &Src,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Self {
        let (base_a_state, stored_cont_a_state) = match force_disable {
            true => (DISABLED_STATE, Some(cont_a_state)),
            false => (cont_a_state, None),
        };
        Self {
            base: UadItemBase::new(item_id, a_item_id, base_a_state, src, reuse_eupdates),
            fit_key,
            cont_key,
            projs: Projs::new(),
            stored_cont_a_state,
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(crate) fn set_a_item_id(&mut self, a_item_id: ad::AItemId, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.set_a_item_id(a_item_id, reuse_eupdates, src);
    }
    pub(crate) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_a_group_id()
    }
    pub(crate) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base.get_a_category_id()
    }
    pub(crate) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        self.base.get_a_attrs()
    }
    pub(crate) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base.get_a_effect_datas()
    }
    pub(crate) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base.get_a_defeff_id()
    }
    pub(crate) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base.get_a_skill_reqs()
    }
    pub(crate) fn get_a_xt(&self) -> Option<&ad::AItemXt> {
        self.base.get_a_xt()
    }
    pub(crate) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(crate) fn set_a_state(&mut self, state: ad::AState, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        match self.stored_cont_a_state {
            // Stored state set means charge is disabled
            Some(_) => {
                self.stored_cont_a_state = Some(state);
                reuse_eupdates.clear();
            }
            // Not disabled - proceed like with any other item
            None => self.base.set_a_state(state, reuse_eupdates, src),
        }
    }
    pub(in crate::uad::item) fn get_reffs(&self) -> Option<&RSet<ad::AEffectId>> {
        self.base.get_reffs()
    }
    pub(in crate::uad::item) fn start_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.start_all_reffs(reuse_eupdates, src);
    }
    pub(in crate::uad::item) fn stop_all_reffs(&self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.stop_all_reffs(reuse_eupdates, src)
    }
    pub(in crate::uad::item) fn get_effect_mode(&self, effect_id: &ad::AEffectId) -> EffectMode {
        self.base.get_effect_mode(effect_id)
    }
    pub(in crate::uad::item) fn set_effect_mode(
        &mut self,
        a_effect_id: ad::AEffectId,
        effect_mode: EffectMode,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_mode(a_effect_id, effect_mode, reuse_eupdates, src)
    }
    pub(in crate::uad::item) fn set_effect_modes(
        &mut self,
        modes: impl Iterator<Item = (ad::AEffectId, EffectMode)>,
        reuse_eupdates: &mut UadEffectUpdates,
        src: &Src,
    ) {
        self.base.set_effect_modes(modes, reuse_eupdates, src)
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        self.base.update_a_data(reuse_eupdates, src);
    }
    pub(crate) fn get_force_disable(&self) -> bool {
        self.stored_cont_a_state.is_some()
    }
    pub(crate) fn set_force_disable(&mut self, force_disable: bool, reuse_eupdates: &mut UadEffectUpdates, src: &Src) {
        match (force_disable, self.stored_cont_a_state) {
            // Attempt to enable when it's already enabled, or disable when it's disabled
            (true, Some(_)) | (false, None) => reuse_eupdates.clear(),
            // Turning force disable on
            (true, None) => {
                self.stored_cont_a_state = Some(self.get_a_state());
                self.base.set_a_state(DISABLED_STATE, reuse_eupdates, src);
            }
            // Turning force disable off
            (false, Some(stored_cont_a_state)) => {
                self.stored_cont_a_state = None;
                self.base.set_a_state(stored_cont_a_state, reuse_eupdates, src);
            }
        }
    }
    pub(crate) fn get_fit_key(&self) -> UadFitKey {
        self.fit_key
    }
    pub(crate) fn get_cont_key(&self) -> UadItemKey {
        self.cont_key
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UadCharge {
    fn get_name() -> &'static str {
        "Charge"
    }
}
impl std::fmt::Display for UadCharge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(item_id={}, a_item_id={})",
            Self::get_name(),
            self.get_item_id(),
            self.get_a_item_id(),
        )
    }
}
