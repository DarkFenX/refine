use crate::{
    def::FitId,
    err::basic::FitFoundError,
    ud::{UFit, UFitKey, container::UEntityContainer},
};

pub(crate) type UFits = UEntityContainer<UFit, UFitKey, FitId, FitFoundError>;
