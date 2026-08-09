use crate::{
    err::basic::{AttrFoundError, ItemFoundError, ItemLoadedError, ItemReceiveProjError, SupportedStatError},
    svc::err::IntStatItemError,
    ud::{ProjecteeUidError, UItems},
};

#[derive(Debug, thiserror::Error)]
pub enum GetItemAttrError {
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error(transparent)]
    AttrNotFound(#[from] AttrFoundError),
}

#[derive(Debug, thiserror::Error)]
pub enum IterItemAttrsError {
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(Debug, thiserror::Error)]
pub enum IterItemEffectsError {
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
}

#[derive(Debug, thiserror::Error)]
pub enum IterItemModifiersError {
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Stats errors
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, thiserror::Error)]
pub enum StatItemError<SS>
where
    SS: std::error::Error,
{
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error(transparent)]
    UnsupportedStat(#[from] SupportedStatError),
    #[error(transparent)]
    StatSpecific(SS),
}
// Conversions
impl<SS> StatItemError<SS>
where
    SS: std::error::Error,
{
    pub(crate) fn from_svc_err(svc_err: IntStatItemError<SS>, u_items: &UItems) -> Self {
        match svc_err {
            IntStatItemError::ItemNotLoaded(svc_err) => ItemLoadedError::from_svc_err(svc_err, u_items).into(),
            IntStatItemError::UnsupportedStat(svc_err) => SupportedStatError::from_svc_err(svc_err, u_items).into(),
            IntStatItemError::StatSpecific(stat_err) => Self::StatSpecific(stat_err),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum StatItemAppliedError<SS>
where
    SS: std::error::Error,
{
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error(transparent)]
    UnsupportedStat(#[from] SupportedStatError),
    #[error(transparent)]
    StatSpecific(SS),
    #[error(transparent)]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
// Conversions
impl<SS> StatItemAppliedError<SS>
where
    SS: std::error::Error,
{
    pub(super) fn from_svc_err(svc_err: IntStatItemError<SS>, u_items: &UItems) -> Self {
        match svc_err {
            IntStatItemError::ItemNotLoaded(svc_err) => ItemLoadedError::from_svc_err(svc_err, u_items).into(),
            IntStatItemError::UnsupportedStat(svc_err) => SupportedStatError::from_svc_err(svc_err, u_items).into(),
            IntStatItemError::StatSpecific(stat_err) => Self::StatSpecific(stat_err),
        }
    }
}
impl<SS> From<ProjecteeUidError> for StatItemAppliedError<SS>
where
    SS: std::error::Error,
{
    fn from(uid_err: ProjecteeUidError) -> Self {
        match uid_err {
            ProjecteeUidError::ProjecteeNotFound(uid_err) => uid_err.into(),
            ProjecteeUidError::ProjecteeCantTakeProjs(uid_err) => uid_err.into(),
        }
    }
}
