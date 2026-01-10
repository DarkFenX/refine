use crate::{
    ad::AAttrId,
    api::{FullMAttr, FullMAttrMut},
    num::UnitInterval,
    sol::SolarSystem,
    ud::UItemId,
};

impl<'a> FullMAttr<'a> {
    /// Return roll quality for the mutated attribute.
    ///
    /// Almost always returns result, even if roll quality wasn't set by the user for the attribute,
    /// in which case roll is based off base attribute value. None is returned only when user roll
    /// is not set, and there is an error with EVE data (e.g. min mult = max mult for the mutation).
    pub fn get_roll(&self) -> Option<UnitInterval> {
        get_roll(self.sol, self.item_uid, &self.attr_aid)
    }
}

impl<'a> FullMAttrMut<'a> {
    /// Return roll quality for the mutated attribute.
    ///
    /// Almost always returns result, even if roll quality wasn't set by the user for the attribute,
    /// in which case roll is based off base attribute value. None is returned only when user roll
    /// is not set, and there is an error with EVE data (e.g. min mult = max mult for the mutation).
    pub fn get_roll(&self) -> Option<UnitInterval> {
        get_roll(self.sol, self.item_uid, &self.attr_aid)
    }
}

fn get_roll(sol: &SolarSystem, item_uid: UItemId, attr_aid: &AAttrId) -> Option<UnitInterval> {
    let u_item = sol.u_data.items.get(item_uid);
    if let Some(&roll) = u_item.get_mutation_data().unwrap().get_attr_rolls().get(attr_aid) {
        return Some(roll);
    }
    // If roll data was not available, calculate it using unmutated attribute value
    let attr_rid = sol.u_data.src.get_attr_rid_by_aid(attr_aid).unwrap();
    let a_mutation_range = u_item
        .get_mutation_data()
        .unwrap()
        .get_cache()
        .unwrap()
        .get_r_mutator()
        .attr_mods
        .get(&attr_rid)
        .unwrap();
    // In absence of mutation, for purposes of calculating roll, it is fine to use base attribute
    // value in place of unmutated attribute value:
    // - in case mutation range includes multiplier of 1, it means base value won't be shifted, an in
    //   absence of mutation - base value matches unmutated value;
    // - if value was shifted into any direction (e.g. unmutated 10 with range [1.2, 1.4] exposed as
    //   base value 12), it will still lie on appropriate edge of shifted roll (in this case it will be
    //   0.0 relatively [14.4, 16.8] range - range is wrong, result is right).
    let value = u_item.get_attr(attr_rid).unwrap();
    let min_value = value * a_mutation_range.min_mult;
    let max_value = value * a_mutation_range.max_mult;
    if min_value == max_value {
        return None;
    }
    let value = (value - min_value) / (max_value - min_value);
    Some(UnitInterval::from_value_clamped(value))
}
