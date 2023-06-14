use std::str::FromStr;

use time::macros::format_description;
use tracing::Level;
use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_subscriber::{
    filter::Targets,
    fmt::{self, time::UtcTime},
    prelude::*,
};

pub(crate) fn setup(folder: Option<String>, level: &str, rotate: bool) -> Option<WorkerGuard> {
    let time_format_full = format_description!(
        version = 2,
        r"\[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]\]"
    );
    // We always log warnings and higher to stdout
    let stdout_log = fmt::layer()
        .with_writer(std::io::stdout.with_max_level(Level::WARN))
        .with_ansi(true)
        .with_timer(UtcTime::new(time_format_full))
        .with_target(false)
        .pretty();
    // We log into file only if we've been given path and appropriate log level
    let file_max_level_res = Level::from_str(level);
    let (file_log, file_guard) = match (folder, file_max_level_res) {
        (Some(folder_path), Ok(max_level)) => {
            let (rotation, time_format) = match rotate {
                true => (
                    tracing_appender::rolling::Rotation::DAILY,
                    format_description!(version = 2, r"\[[hour]:[minute]:[second].[subsecond digits:3]\]"),
                ),
                false => (tracing_appender::rolling::Rotation::NEVER, time_format_full),
            };
            let appender = RollingFileAppender::new(rotation, folder_path, "reefast-http.log");
            let (file_writer, file_guard) = tracing_appender::non_blocking(appender);
            let file_log = fmt::layer()
                .with_writer(file_writer.with_max_level(max_level))
                .with_ansi(false)
                .with_timer(UtcTime::new(time_format))
                .with_target(false)
                .compact();
            (Some(file_log), Some(file_guard))
        }
        _ => (None, None),
    };
    tracing_subscriber::registry()
        .with(stdout_log)
        .with(file_log)
        .with(
            Targets::new()
                .with_default(None)
                .with_target("reefast_core", Level::TRACE)
                .with_target("reefast_dh_eve", Level::TRACE)
                .with_target("reefast_dh_adapted", Level::TRACE)
                .with_target("reefast_http", Level::TRACE)
                .with_target("http", Level::TRACE),
        )
        .init();
    file_guard
}
