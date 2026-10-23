#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
#[derive(Default)]
pub struct ADataWarnings {
    pub data_fetch: Vec<String>,
    pub pk_duplicates: Vec<String>,
    pub cleanup: Vec<String>,
    pub validation: Vec<String>,
    pub conversion_main: Vec<String>,
    pub customization: Vec<String>,
    pub conversion_aux: Vec<String>,
}
impl ADataWarnings {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        let Self {
            data_fetch,
            pk_duplicates,
            cleanup,
            validation,
            conversion_main,
            customization,
            conversion_aux,
        } = self;
        data_fetch.is_empty()
            && pk_duplicates.is_empty()
            && cleanup.is_empty()
            && validation.is_empty()
            && conversion_main.is_empty()
            && customization.is_empty()
            && conversion_aux.is_empty()
    }
}
