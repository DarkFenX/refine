use crate::{
    ad::AAttrId,
    def::AttrId,
    err::basic::{AttrFoundError, ItemMAttrMutatorError, ItemMAttrValueError},
    sol::{
        SolarSystem,
        api::{EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrMut},
    },
    ud::UItemKey,
    util::GetId,
};

impl<'a> EffectiveMutation<'a> {
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr(&self, attr_id: AttrId) -> Result<FullMAttr<'_>, GetFullMAttrError> {
        check_prereqs(self.sol, self.item_key, &attr_id)?;
        Ok(FullMAttr::new(self.sol, self.item_key, attr_id))
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr(&self, attr_id: AttrId) -> Result<FullMAttr<'_>, GetFullMAttrError> {
        check_prereqs(self.sol, self.item_key, &attr_id)?;
        Ok(FullMAttr::new(self.sol, self.item_key, attr_id))
    }
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr_mut(&mut self, attr_id: AttrId) -> Result<FullMAttrMut<'_>, GetFullMAttrError> {
        check_prereqs(self.sol, self.item_key, &attr_id)?;
        Ok(FullMAttrMut::new(self.sol, self.item_key, attr_id))
    }
}

fn check_prereqs(sol: &SolarSystem, item_key: UItemKey, a_attr_id: &AAttrId) -> Result<(), GetFullMAttrError> {
    let u_item = sol.u_data.items.get(item_key);
    let attr_key = match sol.u_data.src.get_attr_key_by_id(a_attr_id) {
        Some(attr_key) => attr_key,
        None => return Err(AttrFoundError { attr_id: *a_attr_id }.into()),
    };
    let mutation_cache = u_item.get_mutation_data().unwrap().get_cache().unwrap();
    if !mutation_cache.get_r_mutator().attr_mods.contains_key(&attr_key) {
        return Err(ItemMAttrMutatorError {
            item_id: sol.u_data.items.id_by_key(item_key),
            attr_id: *a_attr_id,
            mutator_id: mutation_cache.get_r_mutator().get_id(),
        }
        .into());
    };
    if !u_item.get_attrs().unwrap().contains_key(&attr_key) {
        return Err(ItemMAttrValueError {
            item_id: sol.u_data.items.id_by_key(item_key),
            attr_id: *a_attr_id,
        }
        .into());
    };
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum GetFullMAttrError {
    #[error("{0}")]
    AttrNotFound(#[from] AttrFoundError),
    #[error("{0}")]
    NotMutable(#[from] ItemMAttrMutatorError),
    #[error("{0}")]
    NoValue(#[from] ItemMAttrValueError),
}
