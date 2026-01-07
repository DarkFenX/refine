use crate::{
    ad::AAttrId,
    api::{AttrId, EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrMut},
    err::basic::{AttrFoundError, ItemMAttrMutatorError, ItemMAttrValueError},
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> EffectiveMutation<'a> {
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr(&self, attr_id: AttrId) -> Result<FullMAttr<'_>, GetFullMAttrError> {
        let attr_aid = attr_id.into();
        check_prereqs(self.sol, self.item_key, &attr_aid)?;
        Ok(FullMAttr::new(self.sol, self.item_key, attr_aid))
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr(&self, attr_id: AttrId) -> Result<FullMAttr<'_>, GetFullMAttrError> {
        let attr_aid = attr_id.into();
        check_prereqs(self.sol, self.item_key, &attr_aid)?;
        Ok(FullMAttr::new(self.sol, self.item_key, attr_aid))
    }
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr_mut(&mut self, attr_id: AttrId) -> Result<FullMAttrMut<'_>, GetFullMAttrError> {
        let attr_aid = attr_id.into();
        check_prereqs(self.sol, self.item_key, &attr_aid)?;
        Ok(FullMAttrMut::new(self.sol, self.item_key, attr_aid))
    }
}

fn check_prereqs(sol: &SolarSystem, item_key: UItemId, attr_aid: &AAttrId) -> Result<(), GetFullMAttrError> {
    let u_item = sol.u_data.items.get(item_key);
    let attr_key = match sol.u_data.src.get_attr_rid_by_aid(attr_aid) {
        Some(attr_key) => attr_key,
        None => {
            return Err(AttrFoundError {
                attr_id: attr_aid.into(),
            }
            .into());
        }
    };
    let mutation_cache = u_item.get_mutation_data().unwrap().get_cache().unwrap();
    if !mutation_cache.get_r_mutator().attr_mods.contains_key(&attr_key) {
        return Err(ItemMAttrMutatorError {
            item_id: sol.u_data.items.xid_by_iid(item_key),
            attr_id: attr_aid.into(),
            mutator_id: mutation_cache.get_r_mutator().id,
        }
        .into());
    };
    if !u_item.get_attrs().unwrap().contains_key(&attr_key) {
        return Err(ItemMAttrValueError {
            item_id: sol.u_data.items.xid_by_iid(item_key),
            attr_id: attr_aid.into(),
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
