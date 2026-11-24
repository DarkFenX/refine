use crate::{AttrId, ItemTypeId, Src, UnitInterval, Value};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Src {
    /// Roll which the mutator needs to give the attribute of the base type requested value.
    ///
    /// Value is clamped if it is out of bounds allowed by the mutator. None is returned for the
    /// rest of erroneous cases, e.g.:
    /// - mutator cannot be applied to the base type;
    /// - mutator does not change the attribute;
    /// - attribute has no unmutated value;
    /// - attribute mutation range has zero width.
    pub fn get_mutated_attr_roll(
        &self,
        base_type_id: ItemTypeId,
        mutator_type_id: ItemTypeId,
        attr_id: AttrId,
        value: Value,
    ) -> Option<UnitInterval> {
        let mutator_type_aid = mutator_type_id.into_aid();
        let muta_pregen = self.r_data.get_muta_pregen();
        let entry = muta_pregen.get_entry(base_type_id.into_aid(), mutator_type_aid)?;
        let attr_rid = self.r_data.get_attr_rid_by_aid(&attr_id.into_aid())?;
        let r_mutator = self.r_data.get_mutator_by_aid(&mutator_type_aid)?;
        let mutation_range = r_mutator.attr_mods.get(&attr_rid)?;
        let unmutated_value = *muta_pregen.get_merged_attrs(entry.merged_attrs_rid).get(&attr_rid)?;
        mutation_range.resolve_absolute_into_roll(unmutated_value, value)
    }
}
