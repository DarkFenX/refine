use ordered_float::OrderedFloat;

use super::aggregable::Aggregable;
use crate::{def::AttrVal, misc::DmgKinds, util::Limit};

impl Aggregable for DmgKinds<AttrVal> {}

impl Limit for DmgKinds<AttrVal> {
    // No-op, since there is no logic to limit damage depending on target attrs
    fn limit(&mut self, _limit: OrderedFloat<f64>) {}
}
