use crate::misc::PValue;

pub(crate) trait LimitAmount {
    fn limit_amount(&mut self, limit: PValue);
}
