#![allow(warnings, unused)]

use std::{path::PathBuf, thread::sleep, time::Duration};

use chrono;

use reefast::{ch::CacheHandler, ch_impls::json_file, dh::DataHandler, dh_impls::phobos, SrcMgr, VERSION};

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
    let mut srcmgr = SrcMgr::new();
    let dh = Box::new(phobos::PhbFileDHandler::new("/home/dfx/Desktop/phobos_tq_en-us"));
    // let dh = phobos::PhbHttpDHandler::new("http://localhost:8555/").unwrap();
    let mut ch = Box::new(json_file::JsonFileCHandler::new(
        PathBuf::from("/home/dfx/Workspace/eve/reefast/cache/"),
        "tq",
    ));
    srcmgr.add("tq", dh, ch, true);
    // let item = srcmgr.get_default().unwrap().cache_handler.get_item(11184).unwrap();
    //println!("Item with id {} fetched", item.id);
    //sleep(Duration::new(10, 0));
    srcmgr.del("tq");
    //sleep(Duration::new(10, 0));
}
