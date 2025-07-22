use crate::{
    err::basic::FitFoundError,
    uad::{container::UadEntityContainer, fit::UadFit},
};

pub(crate) type UadFits = UadEntityContainer<UadFit, FitFoundError>;
