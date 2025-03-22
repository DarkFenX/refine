use crate::{consts, defs::EAttrId};

pub(in crate::sol::svc::calc::modifier) const SHIP_MASS: EAttrId = consts::attrs::MASS;
pub(in crate::sol::svc::calc::modifier) const SHIP_SPEED: EAttrId = consts::attrs::MAX_VELOCITY;
pub(in crate::sol::svc::calc::modifier) const PROP_THRUST: EAttrId = consts::attrs::SPEED_BOOST_FACTOR;
pub(in crate::sol::svc::calc::modifier) const PROP_BOOST: EAttrId = consts::attrs::SPEED_FACTOR;
