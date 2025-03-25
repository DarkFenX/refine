use crate::{ac, ad};

pub(in crate::sol::svc::calc::modifier) const SHIP_MASS: ad::AAttrId = ac::attrs::MASS;
pub(in crate::sol::svc::calc::modifier) const SHIP_SPEED: ad::AAttrId = ac::attrs::MAX_VELOCITY;
pub(in crate::sol::svc::calc::modifier) const PROP_THRUST: ad::AAttrId = ac::attrs::SPEED_BOOST_FACTOR;
pub(in crate::sol::svc::calc::modifier) const PROP_BOOST: ad::AAttrId = ac::attrs::SPEED_FACTOR;
