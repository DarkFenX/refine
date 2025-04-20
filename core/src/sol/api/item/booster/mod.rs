pub use booster::{Booster, BoosterMut};
pub use get::GetBoosterError;
pub use side_effect::{
    FullSideEffect, FullSideEffectMut, SideEffect, SideEffectIter, SideEffectMut, SideEffectPartialStr, SideEffectStr,
    StubSideEffect, StubSideEffectMut,
};

mod add;
mod booster;
mod fit_iter;
mod get;
mod remove;
mod set_state;
mod side_effect;
