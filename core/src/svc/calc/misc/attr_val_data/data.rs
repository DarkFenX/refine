use std::collections::hash_map::Entry;

use super::attr::AttrEntry;
use crate::{
    rd::RAttrId,
    svc::calc::{CalcAttrVals, ItemAttrPostprocs},
    util::RMap,
};

#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrData {
    data: RMap<RAttrId, AttrEntry>,
}
impl ItemAttrData {
    pub(in crate::svc::calc) fn new() -> Self {
        Self { data: RMap::new() }
    }
    pub(in crate::svc::calc) fn get(&self, attr_rid: &RAttrId) -> Option<&AttrEntry> {
        self.data.get(attr_rid)
    }
    pub(in crate::svc::calc) fn keys(&self) -> impl ExactSizeIterator<Item = &RAttrId> {
        self.data.keys()
    }
    pub(in crate::svc::calc) fn iter(&self) -> impl ExactSizeIterator<Item = (&RAttrId, &AttrEntry)> {
        self.data.iter()
    }
    pub(in crate::svc::calc) fn len(&self) -> usize {
        self.data.len()
    }
    pub(in crate::svc::calc) fn set_value_and_get_pp(
        &mut self,
        attr_rid: RAttrId,
        value: CalcAttrVals,
    ) -> Option<&ItemAttrPostprocs> {
        match self.data.entry(attr_rid) {
            Entry::Occupied(entry) => {
                let attr_entry = entry.into_mut();
                attr_entry.value = Some(value);
                attr_entry.postprocs.as_ref()
            }
            Entry::Vacant(entry) => {
                entry.insert(AttrEntry {
                    value: Some(value),
                    postprocs: None,
                });
                None
            }
        }
    }
    pub(in crate::svc::calc) fn unset_value(&mut self, attr_rid: RAttrId) -> bool {
        match self.data.entry(attr_rid) {
            Entry::Occupied(mut entry) => entry.get_mut().value.take().is_some(),
            Entry::Vacant(_) => false,
        }
    }
    pub(in crate::svc::calc) fn has_value(&self, attr_rid: &RAttrId) -> bool {
        match self.data.get(attr_rid) {
            Some(attr_entry) => attr_entry.value.is_some(),
            None => false,
        }
    }
    pub(in crate::svc::calc) fn reg_postproc(&mut self, attr_rid: RAttrId, postprocs: ItemAttrPostprocs) {
        match self.data.entry(attr_rid) {
            Entry::Occupied(mut entry) => entry.get_mut().postprocs = Some(postprocs),
            Entry::Vacant(entry) => {
                entry.insert(AttrEntry {
                    value: None,
                    postprocs: Some(postprocs),
                });
            }
        }
    }
    pub(in crate::svc::calc) fn unreg_postproc(&mut self, attr_rid: RAttrId) -> bool {
        match self.data.entry(attr_rid) {
            Entry::Occupied(mut entry) => entry.get_mut().postprocs.take().is_some(),
            Entry::Vacant(_) => false,
        }
    }
}
