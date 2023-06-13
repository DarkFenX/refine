use std::str::FromStr;

use tracing_appender::{non_blocking::WorkerGuard, rolling::RollingFileAppender};
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{self, time::UtcTime},
};

pub(crate) fn setup(log_folder: Option<String>, log_level: Option<String>) -> WorkerGuard {
    let (non_blocking, guard) = match log_folder {
        Some(path) => {
            let file_appender = RollingFileAppender::new(
                tracing_appender::rolling::Rotation::DAILY,
                "/home/dfx/Workspace/eve/reefast/http/logs/",
                "reefast-http.log",
            );
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
    let log_level = LevelFilter::from_str(&log_level.unwrap_or("off".to_string())).unwrap_or(LevelFilter::OFF);
    tracing_subscriber::fmt()
        .event_format(log_format)
        .with_max_level(log_level)
        .with_writer(non_blocking)
        .init();
    guard
}
