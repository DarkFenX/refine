use super::event_shared::{CapSimEventCapGain, CapSimEventInjector};
use crate::def::AttrVal;

pub(super) enum CapSimEvent {
    InjectorReady(CapSimEventInjector),
    CapGain(CapSimEventCapGain),
}
impl CapSimEvent {
    pub(super) fn get_time(&self) -> AttrVal {
        match self {
            Self::InjectorReady(event) => event.time,
            Self::CapGain(event) => event.time,
        }
    }
}
