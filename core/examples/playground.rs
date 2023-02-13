#![allow(warnings, unused)]

use std::{path::PathBuf, sync::Arc, thread::sleep, time::Duration};

use chrono;

use reefast::{ch::CacheHandler, ch_impls::json_file, dh::DataHandler, dh_impls::phobos, SolarSystem, SrcMgr, VERSION};

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S%.3f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().unwrap();
    let mut src_mgr = Arc::new(SrcMgr::new());
    let dh = Box::new(phobos::PhbFileDHandler::new("/home/dfx/Desktop/phobos_tq_en-us"));
    // let dh = phobos::PhbHttpDHandler::new("http://localhost:8555/").unwrap();
    let mut ch = Box::new(json_file::JsonFileCHandler::new(
        PathBuf::from("/home/dfx/Workspace/eve/reefast/cache/"),
        "tq",
    ));
    let src = src_mgr.add("tq", dh, ch, true).unwrap();
    let mut sol_sys = SolarSystem::new(src);
    loop {
        let fit = sol_sys.add_fit().unwrap();
        println!("fit ID: {}", fit);
    }
    // let mut fit = Fit::new(Some(sol_sys));
    // fit.set_ship(Some(Ship::new(11184)));

    src_mgr.del("tq");
}
