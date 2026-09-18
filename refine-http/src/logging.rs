use std::str::FromStr;

use time::{format_description::FormatDescriptionV3, macros::format_description};
use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    fmt::{layer, time::UtcTime},
    prelude::*,
};

use crate::settings::SettingsLog;

const STDOUT_LEVEL: LevelFilter = LevelFilter::WARN;

const TIME_FORMAT_FULL: FormatDescriptionV3<'_> = format_description!(
    version = 3,
    r"\[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]\]"
);
const TIME_FORMAT_SHORT: FormatDescriptionV3<'_> =
    format_description!(version = 3, r"\[[hour]:[minute]:[second].[subsecond digits:3]\]");

pub(crate) const RX_PREFIX: &str = ">>> rx";
pub(crate) const TX_PREFIX: &str = "<<< tx";

fn refine_targets(max_level: LevelFilter) -> Targets {
    Targets::new()
        .with_default(None)
        .with_target("refine_rs", max_level)
        .with_target("refine_http", max_level)
}

#[derive(Copy, Clone)]
pub(crate) enum LogBodies {
    Enabled,
    Disabled,
}

pub(crate) fn setup_logging(settings: SettingsLog) -> (Option<WorkerGuard>, LogBodies) {
    // We always log warnings and higher to stdout
    let stdout_log = layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_timer(UtcTime::new(TIME_FORMAT_FULL))
        .with_target(false)
        .pretty()
        .with_filter(refine_targets(STDOUT_LEVEL));
    // We log into file only if we've been given path and appropriate log level
    let (file_log, file_guard, effective_max_level) = match (settings.dir, LevelFilter::from_str(&settings.level)) {
        // Tracing parses empty string into error level for some reason; consider it as "off"
        // instead with the additional check
        (Some(dir), Ok(max_level)) if max_level > LevelFilter::OFF && !settings.level.is_empty() => {
            let (rotation, time_format) = match settings.rotate {
                true => (tracing_appender::rolling::Rotation::DAILY, TIME_FORMAT_SHORT),
                false => (tracing_appender::rolling::Rotation::NEVER, TIME_FORMAT_FULL),
            };
            let appender = RollingFileAppender::new(rotation, dir, "refine-http.log");
            let (file_writer, file_guard) = tracing_appender::non_blocking(appender);
            let file_log = layer()
                .with_writer(file_writer)
                .with_ansi(false)
                .with_timer(UtcTime::new(time_format))
                .with_target(false)
                .with_filter(refine_targets(max_level));
            (Some(file_log), Some(file_guard), max_level)
        }
        _ => (None, None, LevelFilter::OFF),
    };
    tracing_subscriber::registry().with(stdout_log).with(file_log).init();
    // Log bodies only if both conditions (flag which enables it & log level) are met
    let bodies = match settings.bodies {
        true if effective_max_level >= LevelFilter::INFO => LogBodies::Enabled,
        _ => LogBodies::Disabled,
    };
    (file_guard, bodies)
}
