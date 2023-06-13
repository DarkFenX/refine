use std::str::FromStr;

use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{self, time::UtcTime},
};

pub(crate) fn setup(folder: Option<String>, level: Option<String>, rotate: bool) -> WorkerGuard {
    let (non_blocking, guard) = match folder {
        Some(path) => {
            let rotation = match rotate {
                true => tracing_appender::rolling::Rotation::DAILY,
                false => tracing_appender::rolling::Rotation::NEVER,
            };
            let file_appender = RollingFileAppender::new(rotation, path, "reefast-http.log");
            tracing_appender::non_blocking(file_appender)
        }
        None => tracing_appender::non_blocking(std::io::stdout()),
    };
    let time_format = time::macros::format_description!(
        version = 2,
        r"\[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]\]"
    );
    let log_format = fmt::format()
        .with_timer(UtcTime::new(time_format))
        .with_ansi(false)
        .compact();
    let level_filter = LevelFilter::from_str(&level.unwrap_or("off".to_string())).unwrap_or(LevelFilter::OFF);
    tracing_subscriber::fmt()
        .event_format(log_format)
        .with_max_level(level_filter)
        .with_writer(non_blocking)
        .init();
    guard
}
