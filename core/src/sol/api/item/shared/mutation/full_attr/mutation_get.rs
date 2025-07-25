use crate::{
    ad,
    def::AttrId,
    err::basic::{ItemMAttrMutatorError, ItemMAttrValueError},
    sol::{
        SolarSystem,
        api::{EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrMut},
    },
    uad::UadItemKey,
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

fn check_prereqs(sol: &SolarSystem, item_key: UadItemKey, a_attr_id: &ad::AAttrId) -> Result<(), GetFullMAttrError> {
    let uad_item = sol.uad.items.get(item_key);
    let mutation_cache = uad_item.get_mutation_data().unwrap().get_cache().unwrap();
    if !mutation_cache.get_r_mutator().get_attr_mods().contains_key(a_attr_id) {
        return Err(ItemMAttrMutatorError {
            item_id: sol.uad.items.id_by_key(item_key),
            attr_id: *a_attr_id,
            mutator_id: mutation_cache.get_r_mutator().get_id(),
        }
        .into());
    };
    if !uad_item.get_a_attrs().unwrap().contains_key(a_attr_id) {
        return Err(ItemMAttrValueError {
            item_id: sol.uad.items.id_by_key(item_key),
            attr_id: *a_attr_id,
        }
        .into());
    };
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum GetFullMAttrError {
    #[error("{0}")]
    NotMutable(#[from] ItemMAttrMutatorError),
    #[error("{0}")]
    NoValue(#[from] ItemMAttrValueError),
}
