use crate::def::AttrVal;

pub(in crate::svc) trait LimitAmount {
    fn limit_amount(&mut self, limit: AttrVal);
}
