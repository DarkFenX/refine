use ordered_float::OrderedFloat;

use super::aggregable::Aggregable;
use crate::util::Limit;

impl Aggregable for OrderedFloat<f64> {}

impl Limit for OrderedFloat<f64> {
    fn limit(&mut self, limit: OrderedFloat<f64>) {
        *self = OrderedFloat::min(*self, limit);
    }
}
