use super::id::RMutasPregenFlexDataId;
use crate::rd::{RMutasPregenMergedAttrsId, RcItem};

/// Item type a mutator turns a base item type into, with IDs of their pregenerated unmutated data.
pub(crate) struct RMutasPregenEntry {
    pub(crate) mutated_item: RcItem,
    pub(crate) merged_attrs_rid: RMutasPregenMergedAttrsId,
    pub(super) flex_data_rid: RMutasPregenFlexDataId,
}
