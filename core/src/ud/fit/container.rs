use crate::{
    def::FitId,
    err::basic::FitFoundError,
    ud::{UFit, UFitId, container::UEntityContainer},
};

pub(crate) type UFits = UEntityContainer<UFit, FitId, UFitId, FitFoundError>;
