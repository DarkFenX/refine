use crate::misc::Value;

pub(crate) trait LimitAmount {
    fn limit_amount(&mut self, limit: Value);
}
