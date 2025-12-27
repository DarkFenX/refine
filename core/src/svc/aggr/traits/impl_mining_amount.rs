use ordered_float::OrderedFloat;

use super::aggregable::Aggregable;
use crate::{misc::MiningAmount, util::Limit};

impl Aggregable for MiningAmount {}

impl Limit for MiningAmount {
    // No-op, since there is no logic to limit mining amount depending on target attrs
    fn limit(&mut self, _limit: OrderedFloat<f64>) {}
}
