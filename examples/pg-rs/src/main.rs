#![allow(warnings, unused)]

use chrono::Utc;
use rs::{Refine, SrcAlias};
use tracing_subscriber::prelude::*;

fn setup_logger() -> () {
    let time_format_full = time::macros::format_description!(
        version = 2,
        r"\[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]\]"
    );
    // Always log warnings and higher to stdout
    let stdout_log = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout.with_max_level(tracing::Level::TRACE))
        .with_ansi(true)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::new(time_format_full))
        .with_target(false)
        .pretty();
    tracing_subscriber::registry()
        .with(stdout_log)
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_default(tracing::Level::INFO)
                .with_target("refine_core", tracing::Level::TRACE)
                .with_target("refine_dh_eve", tracing::Level::TRACE)
                .with_target("refine_dh_adapted", tracing::Level::TRACE),
        )
        .init();
}

#[tokio::main]
async fn main() {
    let mut refine = Refine::new(None, 2, 4);
    let src = refine
        .create_src("tq".into(), "111".to_string(), "bad_url".to_string(), true)
        .await;
    println!("{}", src.err().unwrap());
}
