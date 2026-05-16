use crate::num::PValue;

pub(in crate::svc) trait GetDuration {
    fn get_duration(&self) -> PValue;
}
