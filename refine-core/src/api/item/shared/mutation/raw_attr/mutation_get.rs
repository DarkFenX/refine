use crate::{
    AttrId, EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, ItemId, Mutation,
    MutationMut, RawMAttr, RawMAttrMut, SolarSystem, ad::AAttrId, ud::UItemId,
};

impl<'s> Mutation<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_raw_mattr(attr_id),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.get_raw_mattr(attr_id),
        }
    }
}

impl<'s> MutationMut<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_raw_mattr(attr_id),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.get_raw_mattr(attr_id),
        }
    }
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr_mut(&mut self, attr_id: AttrId) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_raw_mattr_mut(attr_id),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.get_raw_mattr_mut(attr_id),
        }
    }
}

impl<'s> EffectiveMutation<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_uid, attr_id.into_aid())
    }
}

impl<'s> EffectiveMutationMut<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_uid, attr_id.into_aid())
    }
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr_mut(&mut self, attr_id: AttrId) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
        get_raw_mattr_mut(self.sol, self.item_uid, attr_id.into_aid())
    }
}

impl<'s> IncompleteMutation<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_uid, attr_id.into_aid())
    }
}

impl<'s> IncompleteMutationMut<'s> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_uid, attr_id.into_aid())
    }
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr_mut(&mut self, attr_id: AttrId) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
        get_raw_mattr_mut(self.sol, self.item_uid, attr_id.into_aid())
    }
}

fn get_raw_mattr(sol: &SolarSystem, item_uid: UItemId, attr_aid: AAttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
    match sol
        .u_data
        .items
        .get(item_uid)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&attr_aid)
    {
        Some(_) => Ok(RawMAttr::new(sol, item_uid, attr_aid)),
        None => Err(GetRawMAttrError::MutationNotFound(
            sol.u_data.items.ext_id_by_int_id(item_uid),
            AttrId::from_aid(attr_aid),
        )),
    }
}

fn get_raw_mattr_mut(
    sol: &mut SolarSystem,
    item_uid: UItemId,
    attr_aid: AAttrId,
) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
    match sol
        .u_data
        .items
        .get(item_uid)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&attr_aid)
    {
        Some(_) => Ok(RawMAttrMut::new(sol, item_uid, attr_aid)),
        None => Err(GetRawMAttrError::MutationNotFound(
            sol.u_data.items.ext_id_by_int_id(item_uid),
            AttrId::from_aid(attr_aid),
        )),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetRawMAttrError {
    #[error("attribute {1} on item {0} contains no mutation data")]
    MutationNotFound(ItemId, AttrId),
}
