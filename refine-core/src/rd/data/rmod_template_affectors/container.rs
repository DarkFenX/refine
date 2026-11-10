use crate::{
    ad::AItemId,
    rd::{RAttrId, REffect, REffectId, RItemBase, RItemEffectListId, RcItem},
    svc::calc::RawModTemplates,
    util::{DenseId, PDense, RMap, RSet},
};

/// Raw modifier template affectors.
///
/// Effects with local modifier templates which take affector value from an attribute, per item
/// effect list and attribute.
pub(crate) struct RLocalRmtAffectors {
    // Values are ranges of effect_rids; ranges with equal effects share a segment
    ranges: RMap<(RItemEffectListId, RAttrId), (u32, u32)>,
    effect_rids: Box<[REffectId]>,
}
impl RLocalRmtAffectors {
    pub(crate) fn get(&self, item_base: &RItemBase, attr_rid: RAttrId) -> &[REffectId] {
        match self.ranges.get(&(item_base.effect_list_id, attr_rid)) {
            Some(&(start, end)) => &self.effect_rids[start as usize..end as usize],
            None => &[],
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RLocalRmtAffectors {
    pub(in crate::rd::data) fn new(
        items: &RMap<AItemId, RcItem>,
        effect_list_refs: &[AItemId],
        effects: &PDense<REffectId, REffect>,
        rmod_templates: &RawModTemplates,
    ) -> Self {
        let mut ranges = RMap::new();
        let mut effect_rids = Vec::new();
        let mut attr_rids = RSet::new();
        // Go through each unique effect list
        for (index, item_aid) in effect_list_refs.iter().enumerate() {
            let list_effect_rids = &items.get(item_aid).unwrap().base.effect_rids;
            // For each, get affector attribute IDs of modifier templates (but only for local
            // modifier kind)
            attr_rids.clear();
            for &effect_rid in list_effect_rids.iter() {
                let effect = effects.get(effect_rid).unwrap();
                attr_rids.extend(rmod_templates.iter_effect_local_affector_attr_rids(effect));
            }
            // For each attribute, record effects which use it in its local modifier affectors as a
            // contiguous segment
            for &attr_rid in attr_rids.iter() {
                let start = effect_rids.len() as u32;
                for &effect_rid in list_effect_rids.iter() {
                    let effect = effects.get(effect_rid).unwrap();
                    if rmod_templates
                        .iter_effect_local_affector_attr_rids(effect)
                        .any(|effect_attr_rid| effect_attr_rid == attr_rid)
                    {
                        effect_rids.push(effect_rid);
                    }
                }
                let effect_list_id = RItemEffectListId::new(index);
                ranges.insert((effect_list_id, attr_rid), (start, effect_rids.len() as u32));
            }
        }
        // Store each distinct segment once to save some memory & not slow things down too much
        let mut segment_starts = RMap::new();
        let mut unique_effect_rids = Vec::new();
        for range in ranges.values_mut() {
            let segment = &effect_rids[range.0 as usize..range.1 as usize];
            let start = *segment_starts.entry(segment).or_insert_with(|| {
                let start = unique_effect_rids.len() as u32;
                unique_effect_rids.extend_from_slice(segment);
                start
            });
            *range = (start, start + segment.len() as u32);
        }
        ranges.shrink_to_fit();
        Self {
            ranges,
            effect_rids: unique_effect_rids.into_boxed_slice(),
        }
    }
}
