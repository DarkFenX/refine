use crate::{
    err::basic::FitFoundError,
    uad::{container::EntityContainer, fit::UadFit},
};

pub(crate) type Fits = EntityContainer<UadFit, FitFoundError>;
