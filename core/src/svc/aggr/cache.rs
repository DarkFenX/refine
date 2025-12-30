use crate::{
    def::AttrVal,
    svc::{cycle::CycleDataFull, output::Output},
    util::Extend,
};

struct AggrPartData<T>
where
    T: Copy,
{
    time: AttrVal,
    total_output: Output<T>,
}

impl<T> Extend<Output<T>, AggrPartData<T>> for CycleDataFull
where
    T: Copy,
{
    fn extend(&mut self, extra_data: Output<T>) -> AggrPartData<T> {
        AggrPartData {
            time: self.time,
            total_output: extra_data,
        }
    }
}
