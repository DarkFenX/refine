use crate::{
    ac, ad,
    def::{FitKey, ItemId},
    src::Src,
    uad::item::{EffectModes, ShipKind, UadItemBase, bool_to_state_offline, state_to_bool},
    util::{Named, RMap},
};

#[derive(Clone)]
pub(crate) struct UadShip {
    base: UadItemBase,
    fit_key: FitKey,
    kind: ShipKind,
}
impl UadShip {
    pub(crate) fn new(src: &Src, item_id: ItemId, a_item_id: ad::AItemId, fit_key: FitKey, state: bool) -> Self {
        let mut ship = Self {
            base: UadItemBase::new(src, item_id, a_item_id, bool_to_state_offline(state)),
            fit_key,
            kind: ShipKind::Unknown,
        };
        ship.update_ship_kind();
        ship
    }
    // Item base methods
    pub(crate) fn get_item_id(&self) -> ItemId {
        self.base.get_item_id()
    }
    pub(crate) fn get_a_item_id(&self) -> ad::AItemId {
        self.base.get_a_item_id()
    }
    pub(crate) fn set_a_item_id(&mut self, src: &Src, a_item_id: ad::AItemId) {
        self.base.set_a_item_id_and_reload(src, a_item_id);
        self.update_ship_kind();
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
        self.update_ship_kind();
    }
    // Item-specific methods
    pub(crate) fn get_ship_state(&self) -> bool {
        state_to_bool(self.base.get_a_state())
    }
    pub(crate) fn set_ship_state(&mut self, state: bool) {
        self.base.set_a_state(bool_to_state_offline(state))
    }
    pub(crate) fn get_fit_key(&self) -> FitKey {
        self.fit_key
    }
    pub(crate) fn get_kind(&self) -> ShipKind {
        self.kind
    }
    pub(crate) fn get_disallowed_in_wspace(&self) -> Option<bool> {
        self.base.get_disallowed_in_wspace()
    }
    fn update_ship_kind(&mut self) {
        self.kind = match self.get_a_category_id() {
            Some(ac::itemcats::SHIP) => ShipKind::Ship,
            Some(ac::itemcats::STRUCTURE) => ShipKind::Structure,
            _ => ShipKind::Unknown,
        };
    }
}
impl Named for UadShip {
    fn get_name() -> &'static str {
        "Ship"
    }
}
impl std::fmt::Display for UadShip {
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
