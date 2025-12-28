use super::{aggregable::Aggregable, instance_limit::LimitAmount};
use crate::AttrVal;

impl Aggregable for AttrVal {}

impl LimitAmount for AttrVal {
    fn limit_amount(&mut self, limit: AttrVal) {
        *self = AttrVal::min(*self, limit);
    }
}
