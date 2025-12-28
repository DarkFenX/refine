use crate::{
    def::{AttrVal, OF},
    svc::output::Output,
};

pub(in crate::svc) struct AggrData<T> {
    pub(in crate::svc) amount: T,
    pub(in crate::svc) time: AttrVal,
}
impl<T> AggrData<T>
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

pub(super) struct AggrOutputData<T>
where
    T: Copy,
{
    pub(super) output: Output<T>,
    pub(super) time: AttrVal,
}
