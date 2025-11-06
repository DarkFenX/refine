use super::event_shared::{CapSimEventCapGain, CapSimEventInjectorAvailable};

pub(super) enum CapSimEvent {
    InjectorAvailable(CapSimEventInjectorAvailable),
    CapGain(CapSimEventCapGain),
}
