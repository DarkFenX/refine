use crate::{
    AttrId, DormantMutation, EffectiveMutation, ItemId, Mutation, RawMAttr, ad::AAttrId, sol::CoreInner, ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Mutation<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&mut self, attr_id: AttrId) -> Result<RawMAttr<'_>, RawMAttrGetError> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_raw_mattr(attr_id),
            Self::Dormant(dormant_mutation) => dormant_mutation.get_raw_mattr(attr_id),
        }
    }
}

impl<'s> EffectiveMutation<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&mut self, attr_id: AttrId) -> Result<RawMAttr<'_>, RawMAttrGetError> {
        get_raw_mattr(self.core, self.item_uid, attr_id.into_aid())
    }
}

impl<'s> DormantMutation<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&mut self, attr_id: AttrId) -> Result<RawMAttr<'_>, RawMAttrGetError> {
        get_raw_mattr(self.core, self.item_uid, attr_id.into_aid())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RawMAttrGetError {
    #[error("attribute {1} on item {0} contains no mutation data")]
    MutationNotFound(ItemId, AttrId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_raw_mattr(core: &mut CoreInner, item_uid: UItemId, attr_aid: AAttrId) -> Result<RawMAttr<'_>, RawMAttrGetError> {
    match core
        .u_data
        .items
        .get(item_uid)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&attr_aid)
    {
        Some(..) => Ok(RawMAttr::new(core, item_uid, attr_aid)),
        None => Err(RawMAttrGetError::MutationNotFound(
            core.u_data.items.ext_id_by_int_id(item_uid),
            AttrId::from_aid(attr_aid),
        )),
    }
}
