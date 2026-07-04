use crate::cacher_json::data::AdaptedConv;

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple, Default)]
pub(in crate::cacher_json::data) struct CDataWarnings {
    data_fetch: Vec<String>,
    pk_duplicates: Vec<String>,
    cleanup: Vec<String>,
    validation: Vec<String>,
    conversion_main: Vec<String>,
    customization: Vec<String>,
    conversion_aux: Vec<String>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CDataWarnings {
    type AEntity = rc::ad::ADataWarnings;

    fn from_adapted(a_warnings: &Self::AEntity) -> Self {
        Self {
            data_fetch: a_warnings.data_fetch.clone(),
            pk_duplicates: a_warnings.pk_duplicates.clone(),
            cleanup: a_warnings.cleanup.clone(),
            validation: a_warnings.validation.clone(),
            conversion_main: a_warnings.conversion_main.clone(),
            customization: a_warnings.customization.clone(),
            conversion_aux: a_warnings.conversion_aux.clone(),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            data_fetch: self.data_fetch,
            pk_duplicates: self.pk_duplicates,
            cleanup: self.cleanup,
            validation: self.validation,
            conversion_main: self.conversion_main,
            customization: self.customization,
            conversion_aux: self.conversion_aux,
        }
    }
}
