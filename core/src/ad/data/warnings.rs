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
    pub(in crate::ad::data) fn new() -> Self {
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
}
