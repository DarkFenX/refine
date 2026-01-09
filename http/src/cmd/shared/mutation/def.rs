use serde::Deserialize;
use serde_tuple::Deserialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::cmd::shared::HItemAttrMutationValue;

#[derive(Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HMutationOnAdd {
    Short(i32),
    Full(HItemMutationFull),
}

#[serde_as]
#[derive(Deserialize)]
#[serde(untagged)]
pub(in crate::cmd) enum HMutationOnChange {
    Mutator(i32),
    Attrs(#[serde_as(as = "Map<DisplayFromStr, _>")] Vec<(rc::AttrId, Option<HItemAttrMutationValue>)>),
    MutatorAndAttrs(HItemMutationFull),
}

#[serde_as]
#[derive(Deserialize_tuple)]
pub(in crate::cmd) struct HItemMutationFull {
    pub(in crate::cmd) mutator_id: i32,
    #[serde_as(as = "Option<Map<DisplayFromStr, _>>")]
    pub(in crate::cmd) attrs: Option<Vec<(rc::AttrId, HItemAttrMutationValue)>>,
}

pub(in crate::cmd) fn apply_mattrs_on_add(mut core_mutation: rc::MutationMut, h_full_mutation: &HItemMutationFull) {
    if let Some(attr_mutations) = &h_full_mutation.attrs {
        for (attr_id, h_value) in attr_mutations {
            match h_value {
                HItemAttrMutationValue::Absolute(value) => apply_absolute(&mut core_mutation, *attr_id, *value),
                HItemAttrMutationValue::Roll(roll) => apply_roll(&mut core_mutation, *attr_id, *roll),
            }
        }
    }
}

pub(in crate::cmd) fn apply_mattrs_on_change(
    mut core_mutation: rc::MutationMut,
    h_changed_attrs: &[(rc::AttrId, Option<HItemAttrMutationValue>)],
) {
    for (attr_id, h_value) in h_changed_attrs {
        match h_value {
            Some(HItemAttrMutationValue::Absolute(value)) => apply_absolute(&mut core_mutation, *attr_id, *value),
            Some(HItemAttrMutationValue::Roll(roll)) => apply_roll(&mut core_mutation, *attr_id, *roll),
            None => {
                if let Ok(core_raw_mattr) = core_mutation.get_raw_mattr_mut(*attr_id) {
                    core_raw_mattr.remove();
                }
            }
        }
    }
}

fn apply_absolute(core_mutation: &mut rc::MutationMut, core_attr_id: rc::AttrId, value: f64) {
    // Absolute values can be applied only to effective mutations, via full mutated attributes
    if let rc::MutationMut::Effective(core_effective_mutation) = core_mutation
        && let Ok(mut core_full_mattr) = core_effective_mutation.get_full_mattr_mut(core_attr_id)
    {
        core_full_mattr.set_value(Some(rc::Value::from_f64(value)))
    }
}

fn apply_roll(core_mutation: &mut rc::MutationMut, core_attr_id: rc::AttrId, roll: f64) {
    // Try to get raw attr, if it's not available - add it
    let core_roll = rc::UnitInterval::from_f64_clamped(roll);
    match core_mutation.get_raw_mattr_mut(core_attr_id) {
        Ok(mut core_raw_mattr) => {
            core_raw_mattr.set_roll(core_roll);
        }
        Err(_) => {
            core_mutation.mutate_raw(core_attr_id, core_roll).unwrap();
        }
    };
}
