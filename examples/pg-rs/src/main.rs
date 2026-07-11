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
                .with_target("refine_rs", tracing::Level::TRACE),
        )
        .init();
}

#[tokio::main]
async fn main() {
    setup_logger();
    let mut refine = Refine::new(Some(String::from("./cache/")), 2, 4);
    let edh: Box<dyn rs::EveDataHandler + Send> =
        Box::new(redh::PhbFileEdh::new("/home/dfx/Desktop/phobos_tq_en-us".into()));
    refine.create_src("tq".into(), edh, true).await.unwrap();
    let sol = refine.create_sol(None).await.unwrap();
}
