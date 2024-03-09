use crate::{defs::EAttrId, ec};

pub(in crate::ss::svc::svce_calc::modifier) const SHIP_MASS: EAttrId = ec::attrs::MASS;
pub(in crate::ss::svc::svce_calc::modifier) const SHIP_SPEED: EAttrId = ec::attrs::MAX_VELOCITY;
pub(in crate::ss::svc::svce_calc::modifier) const PROP_THRUST: EAttrId = ec::attrs::SPEED_BOOST_FACTOR;
pub(in crate::ss::svc::svce_calc::modifier) const PROP_BOOST: EAttrId = ec::attrs::SPEED_FACTOR;
