#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
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
        Self {
            data_fetch: Vec::new(),
            pk_duplicates: Vec::new(),
            cleanup: Vec::new(),
            validation: Vec::new(),
            conversion_main: Vec::new(),
            customization: Vec::new(),
            conversion_aux: Vec::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.data_fetch.is_empty()
            && self.pk_duplicates.is_empty()
            && self.cleanup.is_empty()
            && self.validation.is_empty()
            && self.conversion_main.is_empty()
            && self.customization.is_empty()
            && self.conversion_aux.is_empty()
    }
}
