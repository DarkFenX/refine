use crate::{
    err::basic::FitFoundError,
    ud::{FitId, UFit, UFitId, container::UEntityContainer},
};

pub(crate) type UFits = UEntityContainer<UFit, FitId, UFitId, FitFoundError>;
