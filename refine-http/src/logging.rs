use std::str::FromStr;

use time::{format_description::FormatDescriptionV3, macros::format_description};
use tracing::Level;
use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_subscriber::{
    filter::Targets,
    fmt::{layer, time::UtcTime},
    prelude::*,
};

use crate::settings::SettingsLog;

const TIME_FORMAT_FULL: FormatDescriptionV3<'_> = format_description!(
    version = 3,
    r"\[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]\]"
);
const TIME_FORMAT_SHORT: FormatDescriptionV3<'_> =
    format_description!(version = 3, r"\[[hour]:[minute]:[second].[subsecond digits:3]\]");

pub(crate) const RX_PREFIX: &str = ">>> rx";
pub(crate) const TX_PREFIX: &str = "<<< tx";

#[derive(Copy, Clone)]
pub(crate) enum LogBodies {
    Enabled,
    Disabled,
}

pub(crate) fn setup(settings: SettingsLog) -> (Option<WorkerGuard>, LogBodies) {
    // We always log warnings and higher to stdout
    let stdout_log = layer()
        .with_writer(std::io::stdout.with_max_level(Level::WARN))
        .with_ansi(true)
        .with_timer(UtcTime::new(TIME_FORMAT_FULL))
        .with_target(false)
        .pretty();
    // We log into file only if we've been given path and appropriate log level
    let (file_log, file_guard, effective_max_level) = match (settings.dir, Level::from_str(&settings.level)) {
        (Some(dir), Ok(max_level)) => {
            let (rotation, time_format) = match settings.rotate {
                true => (tracing_appender::rolling::Rotation::DAILY, TIME_FORMAT_SHORT),
                false => (tracing_appender::rolling::Rotation::NEVER, TIME_FORMAT_FULL),
            };
            let appender = RollingFileAppender::new(rotation, dir, "refine-http.log");
            let (file_writer, file_guard) = tracing_appender::non_blocking(appender);
            let file_log = layer()
                .with_writer(file_writer.with_max_level(max_level))
                .with_ansi(false)
                .with_timer(UtcTime::new(time_format))
                .with_target(false);
            (Some(file_log), Some(file_guard), Some(max_level))
        }
        _ => (None, None, None),
    };
    tracing_subscriber::registry()
        .with(stdout_log)
        .with(file_log)
        .with(
            Targets::new()
                .with_default(None)
                .with_target("refine_rs", Level::TRACE)
                .with_target("refine_http", Level::TRACE),
        )
        .init();
    // Log bodies only if both conditions (flag which enables it & log level) are met
    let bodies = match (settings.bodies, effective_max_level) {
        (true, Some(effective_max_level)) if effective_max_level >= Level::INFO => LogBodies::Enabled,
        _ => LogBodies::Disabled,
    };
    (file_guard, bodies)
}
