// Data for item count in a group limit - need to do it here for efficiency, and to take into
// account that mutated item can have the limit even if raw mutated type has no such limit

use crate::{
    ad::{AAttrId, ADataGenerator, AItemGrpId},
    util::{RMapRSet, RSet},
};

impl ADataGenerator {
    pub(super) fn fill_max_group_mutations(&mut self) {
        let grp_mutations = self.get_grp_mutations();
        let limited_fitted_grp_aids = self.get_item_grps_with_attr(&grp_mutations, AAttrId::MAX_GROUP_FITTED);
        let limited_online_grp_aids = self.get_item_grps_with_attr(&grp_mutations, AAttrId::MAX_GROUP_ONLINE);
        let limited_active_grp_aids = self.get_item_grps_with_attr(&grp_mutations, AAttrId::MAX_GROUP_ACTIVE);
        for a_item in self.a_data.items.data.values_mut() {
            a_item.val_fitted_group_id = match limited_fitted_grp_aids.contains(&a_item.grp_id) {
                true => Some(a_item.grp_id),
                false => None,
            };
            a_item.val_online_group_id = match limited_online_grp_aids.contains(&a_item.grp_id) {
                true => Some(a_item.grp_id),
                false => None,
            };
            a_item.val_active_group_id = match limited_active_grp_aids.contains(&a_item.grp_id) {
                true => Some(a_item.grp_id),
                false => None,
            };
        }
    }
    fn get_grp_mutations(&self) -> RMapRSet<AItemGrpId, AItemGrpId> {
        // Mutated items can potentially change their group ID during mutation; here, we compose a map
        // between base item group IDs and mutated item group IDs
        let mut mutations = RMapRSet::new();
        for a_muta in self.a_data.mutas.data.values() {
            for a_item_conv in a_muta.item_map.iter() {
                let Some(base_item) = self.a_data.items.data.get(&a_item_conv.base_item_id) else {
                    continue;
                };
                let Some(mutated_item) = self.a_data.items.data.get(&a_item_conv.mutated_item_id) else {
                    continue;
                };
                mutations.add_entry(base_item.grp_id, mutated_item.grp_id);
            }
        }
        mutations
    }
    fn get_item_grps_with_attr(
        &self,
        grp_mutations: &RMapRSet<AItemGrpId, AItemGrpId>,
        attr_id: AAttrId,
    ) -> RSet<AItemGrpId> {
        let mut grp_aids = RSet::new();
        for a_item in self.a_data.items.data.values() {
            if a_item.attrs.contains_id(&attr_id) {
                grp_aids.insert(a_item.grp_id);
                grp_aids.extend(grp_mutations.get(&a_item.grp_id).copied())
            }
        }
        grp_aids
    }
}
