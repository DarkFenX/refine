use crate::util::{HError, HErrorKind, HResult};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct HSetServer {
    pub(crate) port: u16,
    pub(crate) solsys_lifetime: u64,
    pub(crate) solsys_cleanup_interval: u64,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct HSetCache {
    pub(crate) folder: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct HSetLog {
    pub(crate) folder: Option<String>,
    pub(crate) level: String,
    pub(crate) rotate: bool,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct HSettings {
    pub(crate) server: HSetServer,
    pub(crate) cache: HSetCache,
    pub(crate) log: HSetLog,
}
impl HSettings {
    pub(crate) fn new(conf_path: Option<String>) -> HResult<Self> {
        Self::new_internal(conf_path).map_err(|e| HError::new(HErrorKind::SettingsInitFailed(e.to_string())))
    }
    fn new_internal(conf_path_opt: Option<String>) -> Result<Self, config::ConfigError> {
        // Set defaults - in quite a cumbersome way, mostly because config crate does not expose
        // a good way to set defaults for values residing on a level deeper first one
        let mut server_defaults = config::Map::new();
        server_defaults.insert("port".to_string(), config::ValueKind::U64(8000));
        server_defaults.insert("solsys_lifetime".to_string(), config::ValueKind::U64(900));
        server_defaults.insert("solsys_cleanup_interval".to_string(), config::ValueKind::U64(30));
        let mut cache_defaults = config::Map::new();
        cache_defaults.insert("folder".to_string(), config::ValueKind::Nil);
        let mut log_defaults = config::Map::new();
        log_defaults.insert("folder".to_string(), config::ValueKind::Nil);
        log_defaults.insert("level".to_string(), config::ValueKind::String("off".to_string()));
        log_defaults.insert("rotate".to_string(), config::ValueKind::Boolean(false));
        let builder = config::Config::builder()
            .set_default("server", server_defaults)?
            .set_default("cache", cache_defaults)?
            .set_default("log", log_defaults)?;
        // Overwrite defaults with values from file only if we have a path to it
        let builder = match conf_path_opt {
            Some(conf_path) => builder.add_source(config::File::with_name(&conf_path).required(false)),
            None => builder,
        };
        builder.build()?.try_deserialize()
    }
}
