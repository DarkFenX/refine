use crate::{
    ac,
    ad::{AItemId, ASkillLevel},
    util::RMap,
};

pub(super) fn is_ice_harvester(item_srqs: &RMap<AItemId, ASkillLevel>) -> bool {
    item_srqs.contains_key(&ac::items::ICE_HARVESTING)
}
