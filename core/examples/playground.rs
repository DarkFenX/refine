#![allow(warnings, unused)]

use std::{path::PathBuf, sync::Arc, thread::sleep, time::Duration};

use chrono;
use itertools::Itertools;

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
    // Get some data for skills
    let grp_ids = dh
        .get_item_groups()
        .unwrap()
        .data
        .iter()
        .filter(|v| v.category_id == 16)
        .map(|v| v.id)
        .collect_vec();
    let skill_ids = dh
        .get_items()
        .unwrap()
        .data
        .iter()
        .filter(|v| grp_ids.contains(&v.group_id))
        .map(|v| v.id)
        .collect_vec();
    let mut ch = Box::new(json_file::JsonFileCHandler::new(
        PathBuf::from("/home/dfx/Workspace/eve/reefast/cache/"),
        "tq",
    ));
    let src = src_mgr.add("tq", dh, ch, true).unwrap();
    let mut sol_sys = SolarSystem::new(src);
    let fit = sol_sys.add_fit().unwrap();
    let ship = sol_sys.set_ship(fit, 11184).unwrap();
    for skill_id in skill_ids.iter() {
        sol_sys.add_skill(fit, skill_id.to_owned(), 5);
    }
    let implant = sol_sys.add_implant(fit, 19687);
    let maxvel = sol_sys.get_item_attr(&ship, &37).unwrap();
    println!("{maxvel}");
    src_mgr.del("tq");
}
