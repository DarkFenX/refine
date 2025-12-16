use crate::{
    ad::AAttrId,
    err::basic::ItemMAttrFoundError,
    misc::AttrId,
    sol::{
        SolarSystem,
        api::{
            EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut,
            RawMAttr, RawMAttrMut,
        },
    },
    ud::UItemKey,
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
        get_raw_mattr(self.sol, self.item_key, attr_id.into())
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_key, attr_id.into())
    }
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr_mut(&mut self, attr_id: AttrId) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
        get_raw_mattr_mut(self.sol, self.item_key, attr_id.into())
    }
}

impl<'a> IncompleteMutation<'a> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_key, attr_id.into())
    }
}

impl<'a> IncompleteMutationMut<'a> {
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr(&self, attr_id: AttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
        get_raw_mattr(self.sol, self.item_key, attr_id.into())
    }
    /// Get mutation's raw mutated attribute for requested attribute ID.
    pub fn get_raw_mattr_mut(&mut self, attr_id: AttrId) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
        get_raw_mattr_mut(self.sol, self.item_key, attr_id.into())
    }
}

fn get_raw_mattr(sol: &SolarSystem, item_key: UItemKey, a_attr_id: AAttrId) -> Result<RawMAttr<'_>, GetRawMAttrError> {
    match sol
        .u_data
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&a_attr_id)
    {
        Some(_) => Ok(RawMAttr::new(sol, item_key, a_attr_id)),
        None => Err(ItemMAttrFoundError {
            item_id: sol.u_data.items.id_by_key(item_key),
            attr_id: a_attr_id.into(),
        }
        .into()),
    }
}

fn get_raw_mattr_mut(
    sol: &mut SolarSystem,
    item_key: UItemKey,
    a_attr_id: AAttrId,
) -> Result<RawMAttrMut<'_>, GetRawMAttrError> {
    match sol
        .u_data
        .items
        .get(item_key)
        .get_mutation_data()
        .unwrap()
        .get_attr_rolls()
        .get(&a_attr_id)
    {
        Some(_) => Ok(RawMAttrMut::new(sol, item_key, a_attr_id)),
        None => Err(ItemMAttrFoundError {
            item_id: sol.u_data.items.id_by_key(item_key),
            attr_id: a_attr_id.into(),
        }
        .into()),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRawMAttrError {
    #[error("{0}")]
    MutationNotFound(#[from] ItemMAttrFoundError),
}
