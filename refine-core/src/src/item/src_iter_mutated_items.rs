use crate::{ItemTypeId, Src, src::SrcItem};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src {
    /// Yields base item type ID, mutator type ID and the mutated item type they produce.
    pub fn iter_mutated_items(&self) -> impl ExactSizeIterator<Item = (ItemTypeId, ItemTypeId, SrcItem<'_>)> {
        self.r_data
            .get_muta_pregen()
            .iter_keyed_entries()
            .map(|((base_type_aid, mutator_type_aid), entry)| {
                (
                    ItemTypeId::from_aid(base_type_aid),
                    ItemTypeId::from_aid(mutator_type_aid),
                    SrcItem::new(&self.r_data, &entry.mutated_item),
                )
            })
    }
}
