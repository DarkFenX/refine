use crate::{
    ad,
    defs::{AttrVal, EAttrId, EEffectId, EItemGrpId, EItemId, SkillLevel, SolFitId, SolItemId},
    ec,
    sol::uad::item::{SolEffectModes, SolItemBase, SolItemState, SolShipKind, bool_to_state_offline, state_to_bool},
    src::Src,
    util::{Named, StMap},
};

#[derive(Clone)]
pub(in crate::sol) struct SolShip {
    base: SolItemBase,
    fit_id: SolFitId,
    kind: SolShipKind,
}
impl SolShip {
    pub(in crate::sol) fn new(src: &Src, id: SolItemId, type_id: EItemId, fit_id: SolFitId, state: bool) -> Self {
        let mut ship = Self {
            base: SolItemBase::new(src, id, type_id, bool_to_state_offline(state)),
            fit_id,
            kind: SolShipKind::Unknown,
        };
        ship.update_ship_kind();
        ship
    }
    // Item base methods
    pub(in crate::sol) fn get_id(&self) -> SolItemId {
        self.base.get_id()
    }
    pub(in crate::sol) fn get_type_id(&self) -> EItemId {
        self.base.get_type_id()
    }
    pub(in crate::sol) fn get_group_id(&self) -> Option<EItemGrpId> {
        self.base.get_group_id()
    }
    pub(in crate::sol) fn get_category_id(&self) -> Option<EItemGrpId> {
        self.base.get_category_id()
    }
    pub(in crate::sol) fn get_attrs(&self) -> Option<&StMap<EAttrId, AttrVal>> {
        self.base.get_attrs()
    }
    pub(in crate::sol) fn get_effect_datas(&self) -> Option<&StMap<EEffectId, ad::AItemEffectData>> {
        self.base.get_effect_datas()
    }
    pub(in crate::sol) fn get_defeff_id(&self) -> Option<Option<EEffectId>> {
        self.base.get_defeff_id()
    }
    pub(in crate::sol) fn get_skill_reqs(&self) -> Option<&StMap<EItemId, SkillLevel>> {
        self.base.get_skill_reqs()
    }
    pub(in crate::sol) fn get_a_extras(&self) -> Option<&ad::AItemExtras> {
        self.base.get_a_extras()
    }
    pub(in crate::sol) fn get_state(&self) -> SolItemState {
        self.base.get_state()
    }
    pub(in crate::sol) fn get_effect_modes(&self) -> &SolEffectModes {
        self.base.get_effect_modes()
    }
    pub(in crate::sol) fn get_effect_modes_mut(&mut self) -> &mut SolEffectModes {
        self.base.get_effect_modes_mut()
    }
    pub(in crate::sol) fn is_loaded(&self) -> bool {
        self.base.is_loaded()
    }
    pub(in crate::sol::uad::item) fn update_a_data(&mut self, src: &Src) {
        self.base.update_a_data(src);
        self.update_ship_kind();
    }
    // Item-specific methods
    pub(in crate::sol) fn get_ship_state(&self) -> bool {
        state_to_bool(self.base.get_state())
    }
    pub(in crate::sol) fn set_ship_state(&mut self, state: bool) {
        self.base.set_state(bool_to_state_offline(state))
    }
    pub(in crate::sol) fn get_fit_id(&self) -> SolFitId {
        self.fit_id
    }
    pub(in crate::sol) fn get_kind(&self) -> SolShipKind {
        self.kind
    }
    fn update_ship_kind(&mut self) {
        self.kind = match self.get_category_id() {
            Some(ec::itemcats::SHIP) => SolShipKind::Ship,
            Some(ec::itemcats::STRUCTURE) => SolShipKind::Structure,
            _ => SolShipKind::Unknown,
        };
    }
}
impl Named for SolShip {
    fn get_name() -> &'static str {
        "SolShip"
    }
}
impl std::fmt::Display for SolShip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}(id={}, type_id={})",
            Self::get_name(),
            self.get_id(),
            self.get_type_id(),
        )
    }
}
