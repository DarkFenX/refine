use super::attr::AttrEntry;
use crate::{
    ad::AAttrId,
    dbg::DebugResult,
    rd::RAttrId,
    svc::calc::{CalcAttrVals, ItemAttrPostproc},
    util::{RMap, SlabId},
};

const NO_ENTRY: u8 = u8::MAX;
const DENSE_ATTR_COUNT: usize = AAttrId::DENSE_ATTRS.len();

/// Calculated attribute values and registered postprocessors of an item.
///
/// Attributes which are (manually) selected due to supposedly often use are stored densely: their
/// entries are kept in a vector, and their positions in it are looked up by attribute ID. The rest
/// are stored in a map.
#[derive(Clone)]
pub(in crate::svc::calc) struct ItemAttrData {
    dense_positions: [u8; DENSE_ATTR_COUNT],
    dense_entries: Vec<(RAttrId, AttrEntry)>,
    sparse: RMap<RAttrId, AttrEntry>,
}
impl ItemAttrData {
    pub(super) fn from_parts(dense_entries: Vec<(RAttrId, AttrEntry)>, sparse: RMap<RAttrId, AttrEntry>) -> Self {
        Self {
            dense_positions: [NO_ENTRY; DENSE_ATTR_COUNT],
            dense_entries,
            sparse,
        }
    }
    pub(super) fn into_parts(self) -> (Vec<(RAttrId, AttrEntry)>, RMap<RAttrId, AttrEntry>) {
        (self.dense_entries, self.sparse)
    }
    #[inline]
    pub(in crate::svc::calc) fn get(&self, attr_rid: &RAttrId) -> Option<&AttrEntry> {
        let index = attr_rid.index();
        match index < DENSE_ATTR_COUNT {
            true => match self.dense_positions[index] {
                NO_ENTRY => None,
                position => Some(&self.dense_entries[position as usize].1),
            },
            false => self.sparse.get(attr_rid),
        }
    }
    fn get_mut(&mut self, attr_rid: RAttrId) -> Option<&mut AttrEntry> {
        let index = attr_rid.index();
        match index < DENSE_ATTR_COUNT {
            true => match self.dense_positions[index] {
                NO_ENTRY => None,
                position => Some(&mut self.dense_entries[position as usize].1),
            },
            false => self.sparse.get_mut(&attr_rid),
        }
    }
    fn get_or_insert(&mut self, attr_rid: RAttrId) -> &mut AttrEntry {
        let index = attr_rid.index();
        match index < DENSE_ATTR_COUNT {
            true => {
                let position = match self.dense_positions[index] {
                    NO_ENTRY => {
                        let position = self.dense_entries.len();
                        self.dense_entries.push((attr_rid, AttrEntry::default()));
                        self.dense_positions[index] = position as u8;
                        position
                    }
                    position => position as usize,
                };
                &mut self.dense_entries[position].1
            }
            false => self.sparse.entry(attr_rid).or_default(),
        }
    }
    pub(in crate::svc::calc) fn iter(&self) -> impl Iterator<Item = (&RAttrId, &AttrEntry)> {
        self.dense_entries
            .iter()
            .map(|(attr_rid, attr_entry)| (attr_rid, attr_entry))
            .chain(self.sparse.iter())
    }
    pub(in crate::svc::calc) fn len(&self) -> usize {
        self.dense_entries.len() + self.sparse.len()
    }
    pub(in crate::svc::calc) fn get_postproc(&self, attr_rid: RAttrId) -> Option<ItemAttrPostproc> {
        self.get(&attr_rid)?.postproc
    }
    pub(in crate::svc::calc) fn set_value_and_get_postproc(
        &mut self,
        attr_rid: RAttrId,
        value: CalcAttrVals,
    ) -> Option<ItemAttrPostproc> {
        let attr_entry = self.get_or_insert(attr_rid);
        attr_entry.value = Some(value);
        attr_entry.postproc
    }
    pub(in crate::svc::calc) fn unset_value(&mut self, attr_rid: RAttrId) -> bool {
        self.get_mut(attr_rid)
            .is_some_and(|attr_entry| attr_entry.value.take().is_some())
    }
    pub(in crate::svc::calc) fn has_value(&self, attr_rid: &RAttrId) -> bool {
        self.get(attr_rid).is_some_and(|attr_entry| attr_entry.value.is_some())
    }
    pub(in crate::svc::calc) fn reg_postproc(&mut self, attr_rid: RAttrId, postprocs: ItemAttrPostproc) {
        self.get_or_insert(attr_rid).postproc = Some(postprocs);
    }
    pub(in crate::svc::calc) fn unreg_postproc(&mut self, attr_rid: RAttrId) -> bool {
        self.get_mut(attr_rid)
            .is_some_and(|attr_entry| attr_entry.postproc.take().is_some())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debug
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAttrData {
    pub(super) fn consistency_check(&self) -> DebugResult {
        let Self {
            dense_positions,
            dense_entries,
            sparse,
        } = self;
        // Each dense entry has to be referenced by the position of its attribute
        let mut referenced = 0;
        for (index, &position) in dense_positions.iter().enumerate() {
            if position == NO_ENTRY {
                continue;
            }
            match dense_entries.get(position as usize) {
                Some((attr_rid, _)) if attr_rid.index() == index => referenced += 1,
                _ => return Err(Default::default()),
            }
        }
        if referenced != dense_entries.len() {
            return Err(Default::default());
        }
        // Densely stored attributes are never stored in the map
        if sparse.keys().any(|attr_rid| attr_rid.index() < DENSE_ATTR_COUNT) {
            return Err(Default::default());
        }
        Ok(())
    }
}
