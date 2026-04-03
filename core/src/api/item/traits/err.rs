use crate::{
    err::basic::{AttrFoundError, ItemFoundError, ItemLoadedError, ItemReceiveProjError, SupportedStatError},
    svc::err::StatItemCheckError,
    ud::{ProjecteeUidError, UItems},
};

#[derive(thiserror::Error, Debug)]
pub enum GetItemAttrError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    AttrNotFound(#[from] AttrFoundError),
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemAttrsError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemEffectsError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(thiserror::Error, Debug)]
pub enum IterItemModifiersError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(thiserror::Error, Debug)]
pub enum ItemStatError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
}
impl ItemStatError {
    pub(crate) fn from_svc_err(svc_err: StatItemCheckError, u_items: &UItems) -> Self {
        match svc_err {
            StatItemCheckError::ItemNotLoaded(svc_err) => ItemLoadedError::from_svc_err(svc_err, u_items).into(),
            StatItemCheckError::UnsupportedStat(svc_err) => SupportedStatError::from_svc_err(svc_err, u_items).into(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemStatAppliedError {
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
impl ItemStatAppliedError {
    pub(super) fn from_svc_err(svc_err: StatItemCheckError, u_items: &UItems) -> Self {
        match svc_err {
            StatItemCheckError::ItemNotLoaded(svc_err) => ItemLoadedError::from_svc_err(svc_err, u_items).into(),
            StatItemCheckError::UnsupportedStat(svc_err) => SupportedStatError::from_svc_err(svc_err, u_items).into(),
        }
    }
}
impl From<ProjecteeUidError> for ItemStatAppliedError {
    fn from(uid_err: ProjecteeUidError) -> Self {
        match uid_err {
            ProjecteeUidError::ProjecteeNotFound(uid_err) => uid_err.into(),
            ProjecteeUidError::ProjecteeCantTakeProjs(uid_err) => uid_err.into(),
        }
    }
}
