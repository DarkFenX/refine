use crate::def::AttrVal;

pub(in crate::svc) struct AggrData<T> {
    pub(in crate::svc) amount: T,
    pub(in crate::svc) time: AttrVal,
}
impl<T> AggrData<T>
where
    T: std::ops::Div<AttrVal, Output = T>,
{
    pub(in crate::svc) fn get_per_second(self) -> T {
        self.amount / self.time
    }
}
