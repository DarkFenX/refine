use crate::{
    err::basic::FitFoundError,
    ud::{container::UEntityContainer, fit::UFit},
};

pub(crate) type UFits = UEntityContainer<UFit, FitFoundError>;
