use crate::{
    err::basic::FitFoundError,
    sol::uad::{container::EntityContainer, fit::UadFit},
};

pub(in crate::sol) type Fits = EntityContainer<UadFit, FitFoundError>;
