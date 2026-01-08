use crate::{
    ad::AAttrId,
    api::{AttrId, EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrMut, ItemTypeId},
    err::basic::{AttrFoundError, ItemMAttrMutatorError, ItemMAttrValueError},
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> EffectiveMutation<'a> {
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr(&self, attr_id: AttrId) -> Result<FullMAttr<'_>, GetFullMAttrError> {
        let attr_aid = attr_id.into_aid();
        check_prereqs(self.sol, self.item_uid, &attr_aid)?;
        Ok(FullMAttr::new(self.sol, self.item_uid, attr_aid))
    }
}

impl<'a> EffectiveMutationMut<'a> {
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr(&self, attr_id: AttrId) -> Result<FullMAttr<'_>, GetFullMAttrError> {
        let attr_aid = attr_id.into_aid();
        check_prereqs(self.sol, self.item_uid, &attr_aid)?;
        Ok(FullMAttr::new(self.sol, self.item_uid, attr_aid))
    }
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr_mut(&mut self, attr_id: AttrId) -> Result<FullMAttrMut<'_>, GetFullMAttrError> {
        let attr_aid = attr_id.into_aid();
        check_prereqs(self.sol, self.item_uid, &attr_aid)?;
        Ok(FullMAttrMut::new(self.sol, self.item_uid, attr_aid))
    }
}

fn check_prereqs(sol: &SolarSystem, item_uid: UItemId, attr_aid: &AAttrId) -> Result<(), GetFullMAttrError> {
    let u_item = sol.u_data.items.get(item_uid);
    let attr_rid = match sol.u_data.src.get_attr_rid_by_aid(attr_aid) {
        Some(attr_rid) => attr_rid,
        None => {
            return Err(AttrFoundError {
                attr_id: AttrId::from_aid(*attr_aid),
            }
            .into());
        }
    };
    let mutation_cache = u_item.get_mutation_data().unwrap().get_cache().unwrap();
    if !mutation_cache.get_r_mutator().attr_mods.contains_key(&attr_rid) {
        return Err(ItemMAttrMutatorError {
            item_id: sol.u_data.items.xid_by_iid(item_uid),
            attr_id: AttrId::from_aid(*attr_aid),
            mutator_id: ItemTypeId::from_aid(mutation_cache.get_r_mutator().id),
        }
        .into());
    };
    if !u_item.get_attrs().unwrap().contains_key(&attr_rid) {
        return Err(ItemMAttrValueError {
            item_id: sol.u_data.items.xid_by_iid(item_uid),
            attr_id: AttrId::from_aid(*attr_aid),
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
