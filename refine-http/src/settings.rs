use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub(crate) server: SettingsServer,
    pub(crate) cache: SettingsCache,
    pub(crate) log: SettingsLog,
}
impl Settings {
    pub(crate) fn new(conf_path: Option<String>) -> Self {
        Self::new_internal(conf_path).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct SettingsServer {
    pub(crate) port: u16,
    pub(crate) max_request_size: u32,
    pub(crate) sol_lifetime: u64,
    pub(crate) sol_cleanup_interval: u64,
    pub(crate) standard_threads: usize,
    pub(crate) heavy_threads: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SettingsCache {
    pub(crate) dir: Option<std::path::PathBuf>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SettingsLog {
    pub(crate) dir: Option<std::path::PathBuf>,
    pub(crate) level: String,
    pub(crate) bodies: bool,
    pub(crate) rotate: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Settings {
    fn new_internal(conf_path_opt: Option<String>) -> Result<Self, config::ConfigError> {
        // Set defaults
        let mut server_defaults = config::Map::new();
        server_defaults.insert("port".into(), config::ValueKind::U64(8000));
        server_defaults.insert("max_request_size".into(), config::ValueKind::U64(10 * 1024 * 1024));
        server_defaults.insert("sol_lifetime".into(), config::ValueKind::U64(900));
        server_defaults.insert("sol_cleanup_interval".into(), config::ValueKind::U64(30));
        server_defaults.insert("standard_threads".into(), config::ValueKind::U64(2));
        server_defaults.insert("heavy_threads".into(), config::ValueKind::U64(4));
        let mut cache_defaults = config::Map::new();
        cache_defaults.insert("dir".into(), config::ValueKind::Nil);
        let mut log_defaults = config::Map::new();
        log_defaults.insert("dir".into(), config::ValueKind::Nil);
        log_defaults.insert("level".into(), config::ValueKind::String("off".into()));
        log_defaults.insert("bodies".into(), config::ValueKind::Boolean(false));
        log_defaults.insert("rotate".into(), config::ValueKind::Boolean(false));
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
