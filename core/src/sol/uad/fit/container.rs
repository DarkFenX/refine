use crate::{
    err::basic::FitFoundError,
    sol::uad::{container::EntityContainer, fit::Fit},
};

pub(in crate::sol) type Fits = EntityContainer<Fit, FitFoundError>;
