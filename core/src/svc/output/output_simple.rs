use crate::def::AttrVal;

pub(crate) struct OutputSimple<T>
where
    T: Copy + Clone,
{
    pub(crate) amount: T,
    pub(crate) delay: AttrVal,
}
impl<T> OutputSimple<T>
where
    T: Copy + Clone,
{
    pub(in crate::svc) fn get_total(&self) -> T {
        self.amount
    }
    pub(in crate::svc) fn get_max(&self) -> T {
        self.amount
    }
}
