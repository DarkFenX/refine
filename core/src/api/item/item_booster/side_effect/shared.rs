use crate::{
    ad::{AAttrId, AEffectId},
    rd::{REffectId, Src},
};

pub(super) fn get_se_chance_attr_id_by_effect_id(src: &Src, effect_id: &AEffectId) -> Option<AAttrId> {
    let effect_key = src.get_effect_rid_by_aid(effect_id)?;
    get_se_chance_attr_id_by_effect_key(src, effect_key)
}

pub(super) fn get_se_chance_attr_id_by_effect_key(src: &Src, effect_key: REffectId) -> Option<AAttrId> {
    let attr_key = src.get_effect_by_rid(effect_key).chance_attr_rid?;
    Some(src.get_attr_by_rid(attr_key).aid)
}
