use crate::{
    DetectedItemKind, ItemTypeId, ModuleState,
    rd::{RData, RItem},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Item type of a data source.
#[derive(Copy, Clone)]
pub struct SrcItem<'src> {
    r_data: &'src RData,
    r_item: &'src RItem,
}
impl SrcItem<'_> {
    pub fn get_type_id(&self) -> ItemTypeId {
        ItemTypeId::from_aid(self.r_item.base.aid)
    }
    pub fn get_detected_kind(&self) -> Option<DetectedItemKind> {
        self.r_item.base.detected_kind
    }
    /// Highest state a module of this type can be in, for types detected as modules.
    pub fn get_module_max_state(&self) -> Option<ModuleState> {
        match self.r_item.base.detected_kind {
            Some(DetectedItemKind::ModuleHigh | DetectedItemKind::ModuleMid | DetectedItemKind::ModuleLow) => {
                Some(ModuleState::from_r_state(self.r_item.base.max_state))
            }
            _ => None,
        }
    }
    /// Stance types this ship type can use; nothing for anything but a ship with stances.
    pub fn iter_ship_stance_type_ids(&self) -> impl ExactSizeIterator<Item = ItemTypeId> {
        self.r_data
            .get_ship_stance_aids(&self.r_item.base.aid)
            .iter()
            .map(|&stance_aid| ItemTypeId::from_aid(stance_aid))
    }
    /// Subsystem types this ship type can use; nothing for anything but a ship with subsystems.
    pub fn iter_ship_subsystem_type_ids(&self) -> impl ExactSizeIterator<Item = ItemTypeId> {
        self.r_data
            .get_ship_subsystem_aids(&self.r_item.base.aid)
            .iter()
            .map(|&subsystem_aid| ItemTypeId::from_aid(subsystem_aid))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'src> SrcItem<'src> {
    pub(in crate::src) fn new(r_data: &'src RData, r_item: &'src RItem) -> Self {
        Self { r_data, r_item }
    }
}
