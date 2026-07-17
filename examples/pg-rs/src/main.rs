#![allow(warnings, unused)]

use rs::{AdCaching, Refine, src::SrcAlias};
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
    // Initial setup
    let mut refine = Refine::new(AdCaching::Filesystem("./cache/".into()), 2, 4);
    let edh: Box<dyn rs::EveDataHandler + Send> =
        Box::new(redh::PhbFileEdh::new("/home/dfx/Desktop/phobos_tq_en-us".into()));
    refine.add_src("tq", edh, true).await.unwrap();
    // Main part
    let mut sol = refine
        .add_sol(None, rs::AddSolCmd::new().with_sec_zone(rs::SecZone::WSpace))
        .await
        .unwrap();
    let resps = sol
        .change(vec![
            rs::SolAddFleetCmd::new().into(),
            rs::SolAddFitCmd::new()
                .with_fleet_id(rs::FleetIdBackref::Backref(0))
                .into(),
        ])
        .await
        .unwrap();
    let fit = sol.get_fit(resps.get(1).unwrap().get_fit_id().unwrap()).await.unwrap();
    tracing::error!("fit ID {}", fit.get_fit_id());
    // fleet.remove(rs::RemoveFleetCmd::new());
    // Cleanup
    sol.remove();
    refine.get_src(None).await.unwrap().remove().await;
}
