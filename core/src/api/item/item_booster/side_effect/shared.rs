use crate::{
    ad::{AAttrId, AEffectId},
    rd::{REffectId, Src},
};

pub(super) fn get_se_chance_attr_id_by_effect_id(src: &Src, effect_id: &AEffectId) -> Option<AAttrId> {
    let effect_rid = src.get_effect_rid_by_aid(effect_id)?;
    get_se_chance_attr_id_by_effect_rid(src, effect_rid)
}

pub(super) fn get_se_chance_attr_id_by_effect_rid(src: &Src, effect_rid: REffectId) -> Option<AAttrId> {
    let attr_rid = src.get_effect_by_rid(effect_rid).chance_attr_rid?;
    Some(src.get_attr_by_rid(attr_rid).aid)
}
