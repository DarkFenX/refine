pub(crate) use kind::UShipKind;
pub(crate) use main::UShip;

#[cfg(feature = "dev")]
mod debug;
mod kind;
mod main;
