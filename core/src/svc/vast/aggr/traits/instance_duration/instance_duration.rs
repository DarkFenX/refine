use crate::num::PValue;

pub(crate) trait InstanceDuration {
    fn get_duration(&self) -> PValue;
    fn limit_duration(&mut self, duration: PValue);
}
