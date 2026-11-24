pub use rc::src::{SrcOrigin, SrcOriginGeneratedReason, SrcWarnings};

pub use crate::{
    api::{Src, SrcAlias},
    info::src::{
        SrcInfo, SrcInfoEnumCmd, SrcInfoEnumResp, SrcInfoFull, SrcInfoMode, SrcInfoPartial, SrcItemInfo, SrcItemInfos,
        SrcItemInfosCmd, SrcMutatedItemBaseInfo, SrcMutatedItemInfo, SrcMutatedItemInfos, SrcMutatedItemInfosCmd,
        SrcMutation,
    },
};

pub mod err {
    pub use rc::{
        ad::err::{ADataGeneratorCleanupError, ADataGeneratorError},
        ed::err::EveDataHandlerError,
        src::err::SrcInitError,
    };

    pub use crate::api::{SrcAddError, SrcAliasPruneInitError, SrcAliasStrictInitError, SrcGetError, SrcRemoveError};
}
