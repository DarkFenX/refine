use super::{aggregable::Aggregable, instance_limit::LimitAmount};
use crate::{def::AttrVal, misc::MiningAmount};

impl Aggregable for MiningAmount {}

impl LimitAmount for MiningAmount {
    // No-op, since there is no logic to limit mining amount depending on target attrs
    fn limit_amount(&mut self, _limit: AttrVal) {}
}
