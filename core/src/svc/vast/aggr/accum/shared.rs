use crate::{
    num::{Count, PValue, Value},
    svc::{output::Output, vast::aggr::traits::InstanceDuration},
};

pub(in crate::svc::vast) struct SeqAccum<A> {
    pub(in crate::svc::vast) instances: A,
    pub(in crate::svc::vast) time: PValue,
}
impl<A> SeqAccum<A> {
    fn add_instance<T>(&mut self, instance: T, chance_mult: Option<PValue>, count: Count)
    where
        A: SeqInstanceAccum<T>,
    {
        self.instances.add_instance(instance, chance_mult, count);
    }
    pub(in crate::svc::vast) fn add_output_full<T>(
        &mut self,
        output: &Output<T>,
        chance_mult: Option<PValue>,
        repeat_count: Count,
    ) where
        A: SeqInstanceAccum<T>,
        T: Copy,
    {
        self.instances.add_output_full(output, chance_mult, repeat_count);
    }
    pub(in crate::svc::vast) fn add_output_time_limited<T>(
        &mut self,
        output: &Output<T>,
        chance_mult: Option<PValue>,
        repeat_count: Count,
        time_limit: Value,
    ) where
        A: SeqInstanceAccum<T>,
        T: Copy + InstanceDuration,
    {
        self.instances
            .add_output_time_limited(output, chance_mult, repeat_count, time_limit);
    }
}

pub(in crate::svc::vast) trait SeqInstanceAccum<T> {
    fn add_instance(&mut self, instance: T, chance_mult: Option<PValue>, count: Count);
    fn copy_blank(&self) -> Self;
    fn merge(&mut self, other: &Self, count: Count);
    fn add_output_full(&mut self, output: &Output<T>, chance_mult: Option<PValue>, repeat_count: Count)
    where
        T: Copy,
    {
        self.add_instance(
            output.get_instance(),
            chance_mult,
            output.get_instance_count() * repeat_count,
        )
    }
    fn add_output_time_limited(
        &mut self,
        output: &Output<T>,
        chance_mult: Option<PValue>,
        repeat_count: Count,
        time_limit: Value,
    ) where
        T: Copy + InstanceDuration,
    {
        let mut remaining_time = time_limit;
        for mut instance_data in output.into_instance_iter() {
            remaining_time -= instance_data.time_passed;
            let ptime = match remaining_time >= Value::ZERO {
                true => PValue::from_value_unchecked(remaining_time),
                false => break,
            };
            instance_data.instance.limit_duration(ptime);
            self.add_instance(instance_data.instance, chance_mult, repeat_count)
        }
    }
}
