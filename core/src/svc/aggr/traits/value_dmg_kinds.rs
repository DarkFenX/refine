use super::{aggregable::Aggregable, limit_amount::LimitAmount};
use crate::{def::AttrVal, misc::DmgKinds};

impl Aggregable for DmgKinds<AttrVal> {}

impl LimitAmount for DmgKinds<AttrVal> {
    // No-op, since there is no logic to limit damage depending on target attrs
    fn limit_amount(&mut self, _limit: AttrVal) {}
}
