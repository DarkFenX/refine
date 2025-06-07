use crate::{
    ad,
    err::basic::ItemMAttrFoundError,
    sol::{
        AttrId, ItemKey, SolarSystem,
        api::{
            EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut,
            RawMAttr, RawMAttrMut,
        },
    },
};

impl<'a> Mutation<'a> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        match self {
            Self::Effective(effective_mutation) => effective_mutation.get_raw_mattr(attr_id),
            Self::Incomplete(incomplete_mutation) => incomplete_mutation.get_raw_mattr(attr_id),
        }
    }
}

impl<'a> MutationMut<'a> {
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

impl<'a> EffectiveMutation<'a> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_key, attr_id)
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_key, attr_id)
    }
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr_mut(&mut self, attr_id: AttrId) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
        get_raw_mattr_mut(self.sol, self.item_key, attr_id)
    }
}

impl<'a> IncompleteMutation<'a> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_key, attr_id)
    }
}

impl<'a> IncompleteMutationMut<'a> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_key, attr_id)
    }
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr_mut(&mut self, attr_id: AttrId) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
        get_raw_mattr_mut(self.sol, self.item_key, attr_id)
    }
}

fn get_raw_mattr(
    sol: &SolarSystem,
    item_key: ItemKey,
    a_attr_id: ad::AAttrId,
) -> Result<RawMAttr<'_>, GetRawMAttrError> {
    match sol
        .uad
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&a_attr_id)
    {
        Some(_) => Ok(RawMAttr::new(sol, item_key, a_attr_id)),
        None => Err(ItemMAttrFoundError {
            item_id: sol.uad.items.id_by_key(item_key),
            attr_id: a_attr_id,
        }
        .into()),
    }
}

fn get_raw_mattr_mut(
    sol: &mut SolarSystem,
    item_key: ItemKey,
    a_attr_id: ad::AAttrId,
) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
    match sol
        .uad
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&a_attr_id)
    {
        Some(_) => Ok(RawMAttrMut::new(sol, item_key, a_attr_id)),
        None => Err(ItemMAttrFoundError {
            item_id: sol.uad.items.id_by_key(item_key),
            attr_id: a_attr_id,
        }
        .into()),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRawMAttrError {
    #[error("{0}")]
    MutationNotFound(#[from] ItemMAttrFoundError),
}
