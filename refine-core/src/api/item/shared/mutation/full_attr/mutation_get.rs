use crate::{
    AttrId, EffectiveMutation, EffectiveMutationMut, FullMAttr, FullMAttrMut, ItemId, ItemTypeId, SolarSystem,
    ad::AAttrId, err::basic::AttrFoundError, ud::UItemId,
};

impl<'s> EffectiveMutation<'s> {
    /// Get mutation's full mutated attribute for requested attribute ID.
    pub fn get_full_mattr(&self, attr_id: AttrId) -> Result<FullMAttr<'_>, GetFullMAttrError> {
        let attr_aid = attr_id.into_aid();
        check_prereqs(self.sol, self.item_uid, &attr_aid)?;
        Ok(FullMAttr::new(self.sol, self.item_uid, attr_aid))
    }
}

impl<'s> EffectiveMutationMut<'s> {
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
    let Some(attr_rid) = sol.u_data.r_data.get_attr_rid_by_aid(attr_aid) else {
        return Err(AttrFoundError {
            attr_id: AttrId::from_aid(*attr_aid),
        }
        .into());
    };
    let mutation_cache = u_item.get_mutation_data().unwrap().get_cache().unwrap();
    if !mutation_cache.get_r_mutator().attr_mods.contains_key(&attr_rid) {
        return Err(GetFullMAttrError::NotMutable(
            sol.u_data.items.ext_id_by_int_id(item_uid),
            ItemTypeId::from_aid(mutation_cache.get_r_mutator().id),
            AttrId::from_aid(*attr_aid),
        ));
    };
    if !u_item.get_r_item_attr_data().unwrap().attrs.contains_key(&attr_rid) {
        return Err(GetFullMAttrError::NoValue(
            sol.u_data.items.ext_id_by_int_id(item_uid),
            AttrId::from_aid(*attr_aid),
        ));
    };
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum GetFullMAttrError {
    #[error(transparent)]
    AttrNotFound(#[from] AttrFoundError),
    #[error("attribute {2} is not mutable according to mutator {1} on item {0}")]
    NotMutable(ItemId, ItemTypeId, AttrId),
    #[error("attribute {1} has no base value on item {0}")]
    NoValue(ItemId, AttrId),
}
