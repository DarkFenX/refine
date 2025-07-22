use crate::{
    err::basic::FitFoundError,
    uad::{container::UadEntityContainer, fit::UadFit},
};

pub(crate) type Fits = UadEntityContainer<UadFit, FitFoundError>;
