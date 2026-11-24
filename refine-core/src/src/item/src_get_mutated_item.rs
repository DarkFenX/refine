use crate::{ItemTypeId, Src, src::SrcItem};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src {
    pub fn get_mutated_item(&self, base_type_id: ItemTypeId, mutator_type_id: ItemTypeId) -> Option<SrcItem<'_>> {
        let muta_pregen = self.r_data.get_muta_pregen();
        let entry = muta_pregen.get_entry(base_type_id.into_aid(), mutator_type_id.into_aid())?;
        Some(SrcItem::new(&entry.mutated_item))
    }
}
