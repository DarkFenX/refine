use crate::{ItemTypeId, val::ValOptions};

pub struct TryFitItemsCmd {
    type_ids: Vec<ItemTypeId>,
    val_options: ValOptions,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl TryFitItemsCmd {
    pub fn new(val_options: ValOptions) -> Self {
        Self {
            type_ids: Vec::new(),
            val_options,
        }
    }
    pub fn with_type_ids(mut self, type_ids: impl Iterator<Item = ItemTypeId>) -> Self {
        self.type_ids.clear();
        self.type_ids.extend(type_ids);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl TryFitItemsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Vec<ItemTypeId> {
        core_fit.try_fit_items(&self.type_ids, &self.val_options)
    }
}
