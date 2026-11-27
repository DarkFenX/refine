use crate::{ItemTypeId, Src, src::SrcMutatedItemInfo};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src<'_> {
    /// Yields base item type ID and info of the item type a mutator turns it into.
    pub fn iter_mutated_item_infos(&self) -> impl ExactSizeIterator<Item = (ItemTypeId, SrcMutatedItemInfo)> {
        SrcMutatedItemInfo::iter_from_core(self.inner.get_core())
    }
}
