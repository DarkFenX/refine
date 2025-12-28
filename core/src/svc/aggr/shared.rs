use crate::{
    def::{AttrVal, OF},
    svc::output::Output,
};

pub(in crate::svc) struct AggrAmount<T> {
    pub(in crate::svc) amount: T,
    pub(in crate::svc) time: AttrVal,
}
impl<T> AggrAmount<T>
where
    T: std::ops::Div<AttrVal, Output = T>,
{
    pub(super) fn get_ps(self) -> Option<T> {
        if self.time == OF(0.0) {
            return None;
        }
        Some(self.amount / self.time)
    }
}

pub(in crate::svc) struct AggrOutput<T>
where
    T: Copy,
{
    pub(in crate::svc) output: Output<T>,
    pub(in crate::svc) time: AttrVal,
}
