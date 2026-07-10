/// Full version of the library as a string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) const SERVER_TICK_HZ: u8 = 1;
pub(crate) const SERVER_TICK_S: f64 = 1.0 / SERVER_TICK_HZ as f64;

pub(crate) const AU: f64 = 149_597_870_700.0;

pub(crate) const MAX_SUBCAP_MODULE_VOLUME: f64 = 3500.0;
