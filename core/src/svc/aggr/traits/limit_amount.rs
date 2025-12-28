#![allow(private_bounds)]

use crate::def::AttrVal;

pub(in crate::svc::aggr) trait LimitAmount {
    fn limit_amount(&mut self, limit: AttrVal);
}
