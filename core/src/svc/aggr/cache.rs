use crate::{
    def::AttrVal,
    svc::{cycle::CycleDataFull, output::Output},
    util::ConvertExtend,
};

struct AggrPartData<T>
where
    T: Copy,
{
    time: AttrVal,
    total_output: Output<T>,
}

impl<T> ConvertExtend<Output<T>, AggrPartData<T>> for CycleDataFull
where
    T: Copy,
{
    fn convert_extend(self, xt: Output<T>) -> AggrPartData<T> {
        AggrPartData {
            time: self.time,
            total_output: xt,
        }
    }
}
