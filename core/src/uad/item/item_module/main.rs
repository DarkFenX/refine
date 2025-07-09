use crate::{
    ad,
    def::{Count, FitKey, Idx, ItemId, ItemKey, OF},
    err::basic::ItemNotMutatedError,
    misc::{AttrMutationRequest, ItemMutationRequest, ModRack, ModuleState},
    src::Src,
    uad::{
        Uad,
        err::ItemMutatedError,
        item::{EffectModes, ItemMutationData, Projs, UadItemBaseMutable},
    },
    util::{Named, RMap, round},
};

#[derive(Clone)]
pub(crate) struct UadModule {
    base: UadItemBaseMutable,
    fit_key: FitKey,
    rack: ModRack,
    pos: Idx,
    charge_key: Option<ItemKey>,
    projs: Projs,
}
impl UadModule {
    pub(crate) fn new(
        src: &Src,
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_key: FitKey,
        state: ModuleState,
        rack: ModRack,
        pos: Idx,
        mutation: Option<ItemMutationRequest>,
        charge_key: Option<ItemKey>,
    ) -> Self {
        Self {
            base: UadItemBaseMutable::new(src, item_id, a_item_id, state.into(), mutation),
            fit_key,
            rack,
            pos,
            charge_key,
            projs: Projs::new(),
        }
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(crate) fn set_a_item_id(&mut self, src: &Src, a_item_id: ad::AItemId) {
        self.base.set_a_item_id(src, a_item_id);
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
    pub(crate) fn get_max_a_state(&self) -> Option<ad::AState> {
        self.base.get_max_a_state()
    }
    pub(crate) fn get_val_fitted_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_val_fitted_a_group_id()
    }
    pub(crate) fn get_val_online_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_val_online_a_group_id()
    }
    pub(crate) fn get_val_active_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_val_active_a_group_id()
    }
    pub(crate) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(crate) fn get_effect_modes(&self) -> &EffectModes {
        self.base.get_effect_modes()
    }
    pub(crate) fn get_effect_modes_mut(&mut self) -> &mut EffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(crate) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
    }
    // Mutation-specific methods
    pub(crate) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.base.get_mutation_data()
    }
    pub(crate) fn mutate(&mut self, src: &Src, mutation: ItemMutationRequest) -> Result<(), ItemNotMutatedError> {
        self.base.mutate(src, mutation)
    }
    pub(crate) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<Vec<ad::AAttrId>, ItemMutatedError> {
        self.base.change_mutation_attrs(src, attr_mutations)
    }
    pub(crate) fn set_a_mutator_id(&mut self, src: &Src, a_mutator_id: ad::AItemId) -> Result<(), ItemMutatedError> {
        self.base.set_a_mutator_id(src, a_mutator_id)
    }
    pub(crate) fn unmutate(&mut self, src: &Src) -> Result<(), ItemMutatedError> {
        self.base.unmutate(src)
    }
    // Item-specific methods
    pub(crate) fn get_module_state(&self) -> ModuleState {
        self.base.get_a_state().into()
    }
    pub(crate) fn set_module_state(&mut self, state: ModuleState) {
        self.base.set_a_state(state.into())
    }
    pub(crate) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
    pub(crate) fn get_rack(&self) -> ModRack {
        self.rack
    }
    pub(crate) fn get_pos(&self) -> Idx {
        self.pos
    }
    pub(crate) fn set_pos(&mut self, pos: Idx) {
        self.pos = pos
    }
    pub(crate) fn get_charge_key(&self) -> Option<ItemKey> {
        self.charge_key
    }
    pub(crate) fn set_charge_key(&mut self, charge_key: Option<ItemKey>) {
        self.charge_key = charge_key
    }
    pub(crate) fn get_charge_count(&self, uad: &Uad) -> Option<Count> {
        // No charge - no info
        let charge_key = self.get_charge_key()?;
        let charge_item = uad.items.get(charge_key);
        let module_capacity = match self.get_a_xt() {
            Some(a_xt) => a_xt.capacity,
            // Module not loaded - no info
            _ => {
                return None;
            }
        };
        let charge_volume = match charge_item.get_a_xt() {
            Some(a_xt) if a_xt.volume != OF(0.0) => a_xt.volume,
            // Charge not loaded or has 0 volume - no info
            _ => {
                return None;
            }
        };
        // Rounding is protection against cases like 2.3 / 0.1 = 22.999999999999996
        let charge_count = round(module_capacity / charge_volume, 10).floor() as Count;
        Some(charge_count)
    }
    pub(crate) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(crate) fn get_projs_mut(&mut self) -> &mut Projs {
        &mut self.projs
    }
}
impl Named for UadModule {
    fn get_name() -> &'static str {
        "Module"
    }
}
impl std::fmt::Display for UadModule {
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
