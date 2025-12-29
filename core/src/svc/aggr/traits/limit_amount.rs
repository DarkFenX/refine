use crate::def::AttrVal;

pub(crate) trait LimitAmount {
    fn limit_amount(&mut self, limit: AttrVal);
}
