use crate::{ItemTypeId, ModuleState, rd::RItem, val::DetectedItemKind};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Item type of a data source.
#[derive(Copy, Clone)]
pub struct SrcItem<'src> {
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'src> SrcItem<'src> {
    pub(super) fn new(r_item: &'src RItem) -> Self {
        Self { r_item }
    }
}
