use crate::util::{Error, ErrorKind, Result};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Server {
    pub(crate) port: u16,
    pub(crate) solsys_lifetime: u64,
    pub(crate) solsys_cleanup_interval: u64,
    pub(crate) cache_folder: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Settings {
    pub(crate) server: Server,
}
impl Settings {
    pub(crate) fn new(conf_path: Option<String>) -> Result<Self> {
        Self::new_internal(conf_path)
            .map_err(|e| Error::new(ErrorKind::SettingsInitFailed(e.to_string())))
    }
    fn new_internal(conf_path: Option<String>) -> std::result::Result<Self, config::ConfigError> {
        // Set defaults - in quite a cumbersome way, mostly because config crate does not expose
        // a good way to set defaults for values deeper first level
        let mut server_defaults = config::Map::new();
        server_defaults.insert("port".to_string(), config::ValueKind::U64(8000));
        server_defaults.insert("solsys_lifetime".to_string(), config::ValueKind::U64(900));
        server_defaults.insert("solsys_cleanup_interval".to_string(), config::ValueKind::U64(30));
        server_defaults.insert("cache_folder".to_string(), config::ValueKind::Nil);
        let s = config::Config::builder().set_default("server", server_defaults)?;
        // Overwrite defaults with values from file only if we have a path to it
        let s = match conf_path {
            Some(cp) => s.add_source(config::File::with_name(&cp).required(false)),
            None => s,
        };
        s.build()?.try_deserialize()
    }
}
