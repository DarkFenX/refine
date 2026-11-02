use crate::{
    Value,
    ad::{AAttrId, AItemId},
    rd::{RAttrConsts, RAttrId, RItemFlexData, RcItem, RcMuta},
    util::{DenseId, RDedupMap, RMap},
};

// IDs are u32 to keep key-to-ID maps and mutation caches smaller
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct RMutaPregenMergedAttrsId(u32);
impl DenseId for RMutaPregenMergedAttrsId {
    fn new(index: usize) -> Self {
        Self(index.try_into().unwrap())
    }
    fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct RMutaPregenFlexDataId(u32);
impl DenseId for RMutaPregenFlexDataId {
    fn new(index: usize) -> Self {
        Self(index.try_into().unwrap())
    }
    fn index(self) -> usize {
        self.0 as usize
    }
}

/// Unmutated data of mutated items for every base item and mutator combination.
pub(crate) struct RMutaPregen {
    // Keys are base item and mutator IDs
    merged_attrs: RDedupMap<RMutaPregenMergedAttrsId, (AItemId, AItemId), RMap<RAttrId, Value>>,
    flex_data: RDedupMap<RMutaPregenFlexDataId, (AItemId, AItemId), RItemFlexData>,
}
impl RMutaPregen {
    pub(crate) fn get_merged_attrs(&self, attrs_rid: RMutaPregenMergedAttrsId) -> &RMap<RAttrId, Value> {
        self.merged_attrs.get_by_id(attrs_rid)
    }
    pub(crate) fn get_merged_attrs_rid_and_flex_data(
        &self,
        base_type_aid: AItemId,
        mutator_type_aid: AItemId,
    ) -> Option<(RMutaPregenMergedAttrsId, RItemFlexData)> {
        let key = (base_type_aid, mutator_type_aid);
        let attrs_rid = self.merged_attrs.get_id(&key)?;
        let attrs = self.merged_attrs.get_by_id(attrs_rid);
        let derived = self.flex_data.get(&key).unwrap();
        Some((attrs_rid, derived.with_attrs(attrs.clone())))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RMutaPregen {
    pub(in crate::rd::data) fn new(
        items: &RMap<AItemId, RcItem>,
        mutas: &RMap<AItemId, RcMuta>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        attr_consts: &RAttrConsts,
    ) -> Self {
        let mut attrs = RDedupMap::new();
        let mut derived = RDedupMap::new();
        for r_muta in mutas.values() {
            for (base_type_aid, mutated_type_aid) in r_muta.item_map.iter() {
                let Some(mutated_r_item) = items.get(mutated_type_aid) else {
                    continue;
                };
                let mut merged_attrs = match items.get(base_type_aid) {
                    Some(base_r_item) => {
                        let mut merged_attrs = base_r_item.attr_data.attrs.clone();
                        // Base item attributes have priority in case of collisions
                        for (&attr_rid, &attr_val) in mutated_r_item.attr_data.attrs.iter() {
                            merged_attrs.entry(attr_rid).or_insert(attr_val);
                        }
                        merged_attrs
                    }
                    None => mutated_r_item.attr_data.attrs.clone(),
                };
                merged_attrs.shrink_to_fit();
                let mut merged_data = RItemFlexData::from_attrs(merged_attrs, attr_aid_rid_map, attr_consts);
                let key = (*base_type_aid, r_muta.id);
                attrs.insert(key, std::mem::take(&mut merged_data.attrs));
                derived.insert(key, merged_data);
            }
        }
        attrs.shrink_to_fit();
        derived.shrink_to_fit();
        Self {
            merged_attrs: attrs,
            flex_data: derived,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "dev")]
impl RMutaPregen {
    pub(crate) fn get_merged_attrs_rid(
        &self,
        base_type_aid: AItemId,
        mutator_type_aid: AItemId,
    ) -> Option<RMutaPregenMergedAttrsId> {
        self.merged_attrs.get_id(&(base_type_aid, mutator_type_aid))
    }
}
