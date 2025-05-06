use crate::{
    ad,
    err::basic::ItemNotMutatedError,
    sol::{
        FitKey, Idx, ItemAddMutation, ItemChangeAttrMutation, ItemId, ItemKey, ModRack,
        err::ItemMutatedError,
        uad::item::{EffectModes, ItemMutationData, ModuleState, Projs, UadItemBaseMutable},
    },
    src::Src,
    util::{Named, RMap},
};

#[derive(Clone)]
pub(in crate::sol) struct UadModule {
    base: UadItemBaseMutable,
    fit_key: FitKey,
    rack: ModRack,
    pos: Idx,
    charge_item_key: Option<ItemKey>,
    projs: Projs,
}
impl UadModule {
    pub(in crate::sol) fn new(
        src: &Src,
        item_id: ItemId,
        a_item_id: ad::AItemId,
        fit_key: FitKey,
        state: ModuleState,
        rack: ModRack,
        pos: Idx,
        mutation: Option<ItemAddMutation>,
        charge_item_key: Option<ItemKey>,
    ) -> Self {
        Self {
            base: UadItemBaseMutable::new(src, item_id, a_item_id, state.into(), mutation),
            fit_key,
            rack,
            pos,
            charge_item_key,
            projs: Projs::new(),
        }
    }
    // Item base methods
    pub(in crate::sol) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(in crate::sol) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(in crate::sol) fn get_a_group_id(&self) -> Option<ad::AItemGrpId> {
        self.base.get_a_group_id()
    }
    pub(in crate::sol) fn get_a_category_id(&self) -> Option<ad::AItemCatId> {
        self.base.get_a_category_id()
    }
    pub(in crate::sol) fn get_a_attrs(&self) -> Option<&RMap<ad::AAttrId, ad::AAttrVal>> {
        self.base.get_a_attrs()
    }
    pub(in crate::sol) fn get_a_effect_datas(&self) -> Option<&RMap<ad::AEffectId, ad::AItemEffectData>> {
        self.base.get_a_effect_datas()
    }
    pub(in crate::sol) fn get_a_defeff_id(&self) -> Option<Option<ad::AEffectId>> {
        self.base.get_a_defeff_id()
    }
    pub(in crate::sol) fn get_a_skill_reqs(&self) -> Option<&RMap<ad::AItemId, ad::ASkillLevel>> {
        self.base.get_a_skill_reqs()
    }
    pub(in crate::sol) fn get_a_extras(&self) -> Option<&ad::AItemExtras> {
        self.base.get_a_extras()
    }
    pub(in crate::sol) fn get_a_state(&self) -> ad::AState {
        self.base.get_a_state()
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &EffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut EffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
    }
    // Mutation-specific methods
    pub(in crate::sol) fn get_mutation_data(&self) -> Option<&ItemMutationData> {
        self.base.get_mutation_data()
    }
    pub(in crate::sol) fn mutate(&mut self, src: &Src, mutation: ItemAddMutation) -> Result<(), ItemNotMutatedError> {
        self.base.mutate(src, mutation)
    }
    pub(in crate::sol) fn change_mutation_attrs(
        &mut self,
        src: &Src,
        attr_mutations: Vec<ItemChangeAttrMutation>,
    ) -> Result<Vec<ad::AAttrId>, ItemMutatedError> {
        self.base.change_mutation_attrs(src, attr_mutations)
    }
    pub(in crate::sol) fn unmutate(&mut self, src: &Src) -> Result<(), ItemMutatedError> {
        self.base.unmutate(src)
    }
    // Item-specific methods
    pub(in crate::sol) fn get_module_state(&self) -> ModuleState {
        self.base.get_a_state().into()
    }
    pub(in crate::sol) fn set_module_state(&mut self, state: ModuleState) {
        self.base.set_a_state(state.into())
    }
    pub(in crate::sol) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
    pub(in crate::sol) fn get_rack(&self) -> ModRack {
        self.rack
    }
    pub(in crate::sol) fn get_pos(&self) -> Idx {
        self.pos
    }
    pub(in crate::sol) fn set_pos(&mut self, pos: Idx) {
        self.pos = pos
    }
    pub(in crate::sol) fn get_charge_item_key(&self) -> Option<ItemKey> {
        self.charge_item_key
    }
    pub(in crate::sol) fn set_charge_item_key(&mut self, charge_item_key: Option<ItemKey>) {
        self.charge_item_key = charge_item_key
    }
    pub(in crate::sol) fn get_projs(&self) -> &Projs {
        &self.projs
    }
    pub(in crate::sol) fn get_projs_mut(&mut self) -> &mut Projs {
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
