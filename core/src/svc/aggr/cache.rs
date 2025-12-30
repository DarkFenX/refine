use crate::{
    def::{AttrVal, OF},
    svc::{cycle::CycleDataFull, output::Output},
    util::ConvertExtend,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) struct AggrPartData<T>
where
    T: Copy,
{
    // Time it takes per cycle in this part
    pub(super) time: AttrVal,
    // After "time" part is complete, it takes this time to finish with output
    pub(super) extra_wait_time: AttrVal,
    pub(super) output: Output<T>,
}

impl<T> ConvertExtend<Output<T>, AggrPartData<T>> for CycleDataFull
where
    T: Copy,
{
    fn convert_extend(self, xt: Output<T>) -> AggrPartData<T> {
        AggrPartData {
            time: self.time,
            extra_wait_time: (xt.get_completion_time() - self.time).max(OF(0.0)),
            output: xt,
        }
    }
}
