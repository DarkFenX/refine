use crate::{
    err::basic::{AttrFoundError, ItemFoundError, ItemLoadedError, ItemReceiveProjError, SupportedStatError},
    stats::err::StatError,
    svc::err::IntItemStatError,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Stats errors
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(thiserror::Error, Debug)]
pub enum ItemStatError<SS>
where
    SS: StatError,
{
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
    #[error("{0}")]
    StatSpecific(#[source] SS),
}
impl<SS> StatError for ItemStatError<SS>
where
    SS: StatError + 'static,
{
    fn is_fatal(&self) -> bool {
        match self {
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(err) => err.is_fatal(),
        }
    }
}
impl<SS> ItemStatError<SS>
where
    SS: StatError,
{
    pub(crate) fn from_svc_err(svc_err: IntItemStatError<SS>, u_items: &UItems) -> Self {
        match svc_err {
            IntItemStatError::ItemNotLoaded(svc_err) => ItemLoadedError::from_svc_err(svc_err, u_items).into(),
            IntItemStatError::UnsupportedStat(svc_err) => SupportedStatError::from_svc_err(svc_err, u_items).into(),
            IntItemStatError::StatSpecific(stat_err) => Self::StatSpecific(stat_err),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemAppliedStatError<SS>
where
    SS: StatError,
{
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
    #[error("{0}")]
    StatSpecific(#[source] SS),
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
impl<SS> StatError for ItemAppliedStatError<SS>
where
    SS: StatError + 'static,
{
    fn is_fatal(&self) -> bool {
        match self {
            Self::ItemNotLoaded(_) => true,
            Self::UnsupportedStat(_) => true,
            Self::StatSpecific(err) => err.is_fatal(),
            Self::ProjecteeNotFound(_) => false,
            Self::ProjecteeCantTakeProjs(_) => false,
        }
    }
}
// Conversions
impl<SS> ItemAppliedStatError<SS>
where
    SS: StatError,
{
    pub(super) fn from_svc_err(svc_err: IntItemStatError<SS>, u_items: &UItems) -> Self {
        match svc_err {
            IntItemStatError::ItemNotLoaded(svc_err) => ItemLoadedError::from_svc_err(svc_err, u_items).into(),
            IntItemStatError::UnsupportedStat(svc_err) => SupportedStatError::from_svc_err(svc_err, u_items).into(),
            IntItemStatError::StatSpecific(stat_err) => Self::StatSpecific(stat_err),
        }
    }
}
impl<SS> From<ProjecteeUidError> for ItemAppliedStatError<SS>
where
    SS: StatError,
{
    fn from(uid_err: ProjecteeUidError) -> Self {
        match uid_err {
            ProjecteeUidError::ProjecteeNotFound(uid_err) => uid_err.into(),
            ProjecteeUidError::ProjecteeCantTakeProjs(uid_err) => uid_err.into(),
        }
    }
}
