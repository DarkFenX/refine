use super::{entry::RMutasPregenEntry, id::RMutasPregenFlexDataId};
use crate::{
    Value,
    ad::{AAttrId, AItemId, AMuta},
    rd::{RAttrConsts, RAttrId, RItemFlexData, RMutasPregenMergedAttrsId, RcItem},
    util::{DenseId, RMap},
};

/// Unmutated data of mutated items for every base item and mutator combination.
pub(crate) struct RMutasPregen {
    // Keys are base item and mutator IDs
    entries: RMap<(AItemId, AItemId), RMutasPregenEntry>,
    merged_attrs: Box<[RMap<RAttrId, Value>]>,
    flex_data: Box<[RItemFlexData]>,
}
impl RMutasPregen {
    /// Entries exist only for conversions whose base and mutated item types are both loaded.
    pub(crate) fn get_entry(&self, base_type_aid: AItemId, mutator_type_aid: AItemId) -> Option<&RMutasPregenEntry> {
        self.entries.get(&(base_type_aid, mutator_type_aid))
    }
    pub(crate) fn get_merged_attrs(&self, attrs_rid: RMutasPregenMergedAttrsId) -> &RMap<RAttrId, Value> {
        &self.merged_attrs[attrs_rid.index()]
    }
    pub(crate) fn get_unmutated_flex_data(&self, entry: &RMutasPregenEntry) -> RItemFlexData {
        let attrs = self.get_merged_attrs(entry.merged_attrs_rid);
        self.get_flex_data(entry).with_attrs(attrs.clone())
    }
    pub(crate) fn is_same_flex_data(&self, entry1: &RMutasPregenEntry, entry2: &RMutasPregenEntry) -> bool {
        entry1.flex_data_rid == entry2.flex_data_rid
    }
    pub(in crate::rd::data) fn iter_entries(&self) -> impl ExactSizeIterator<Item = &RMutasPregenEntry> {
        self.entries.values()
    }
    /// Data derived from unmutated attributes, without the attributes themselves.
    pub(in crate::rd::data) fn get_flex_data(&self, entry: &RMutasPregenEntry) -> &RItemFlexData {
        &self.flex_data[entry.flex_data_rid.index()]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RMutasPregen {
    pub(in crate::rd::data) fn new(
        items: &RMap<AItemId, RcItem>,
        a_mutas: &RMap<AItemId, AMuta>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        attr_consts: &RAttrConsts,
    ) -> Self {
        let mut entries = RMap::new();
        let mut merged_attrs = Vec::new();
        let mut flex_data = Vec::new();
        for a_muta in a_mutas.values() {
            // When a base item is listed several times, the last mutated item is used
            let item_map: RMap<_, _> = a_muta
                .items
                .iter()
                .map(|v| (v.base_item_id, v.mutated_item_id))
                .collect();
            for (base_type_aid, mutated_type_aid) in item_map.into_iter() {
                let Some(base_r_item) = items.get(&base_type_aid) else {
                    continue;
                };
                let Some(mutated_r_item) = items.get(&mutated_type_aid) else {
                    continue;
                };
                let mut entry_merged_attrs = base_r_item.flex_data.attrs.clone();
                // Base item attributes have priority in case of collisions
                for (&attr_rid, &attr_val) in mutated_r_item.flex_data.attrs.iter() {
                    entry_merged_attrs.entry(attr_rid).or_insert(attr_val);
                }
                entry_merged_attrs.shrink_to_fit();
                let mut entry_flex_data = RItemFlexData::from_attrs(entry_merged_attrs, attr_aid_rid_map, attr_consts);
                let entry_merged_attrs = std::mem::take(&mut entry_flex_data.attrs);
                let entry = RMutasPregenEntry {
                    mutated_item: mutated_r_item.clone(),
                    merged_attrs_rid: RMutasPregenMergedAttrsId::new(push_dedup(&mut merged_attrs, entry_merged_attrs)),
                    flex_data_rid: RMutasPregenFlexDataId::new(push_dedup(&mut flex_data, entry_flex_data)),
                };
                entries.insert((base_type_aid, a_muta.id), entry);
            }
        }
        entries.shrink_to_fit();
        Self {
            entries,
            merged_attrs: merged_attrs.into_boxed_slice(),
            flex_data: flex_data.into_boxed_slice(),
        }
    }
}

// Equal values are stored once; there are few distinct values, so linear search is fine
fn push_dedup<V>(values: &mut Vec<V>, value: V) -> usize
where
    V: PartialEq,
{
    match values.iter().position(|v| *v == value) {
        Some(index) => index,
        None => {
            values.push(value);
            values.len() - 1
        }
    }
}
