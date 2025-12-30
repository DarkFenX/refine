use crate::{
    def::AttrVal,
    svc::{cycle::CycleDataFull, output::Output},
    util::ConvertExtend,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartData<T>
where
    T: Copy,
{
    pub(super) time: AttrVal,
    pub(super) output: Output<T>,
}

impl<T> ConvertExtend<Output<T>, AggrPartData<T>> for CycleDataFull
where
    T: Copy,
{
    fn convert_extend(self, xt: Output<T>) -> AggrPartData<T> {
        AggrPartData {
            time: self.time,
            output: xt,
        }
    }
}
