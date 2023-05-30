#![allow(warnings, unused)]

use std::{path::PathBuf, sync::Arc, thread::sleep, time::Duration};

use chrono;
use itertools::Itertools;

use reefast::{ch::CacheHandler, ch_impls, dh::DataHandler, dh_impls, SolarSystem, Src, VERSION};

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
    let dh = Box::new(dh_impls::PhbFileDHandler::new(
        "/home/dfx/Desktop/phobos_tq_en-us".into(),
    ));
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
    let mut ch = Box::new(ch_impls::JsonFileCHandler::new(
        PathBuf::from("/home/dfx/Workspace/eve/reefast/cache/"),
        "tq".to_string(),
    ));
    let src = Arc::new(Src::new(dh, ch).unwrap());
    let mut sol_sys = SolarSystem::new(src);
    let fit = sol_sys.add_fit().unwrap();
    let ship = sol_sys.set_fit_ship(fit, 11184, true).unwrap();
    // for skill_id in skill_ids.iter() {
    //     sol_sys.add_skill(fit, skill_id.to_owned(), 5, true);
    // }
    let implant = sol_sys.add_implant(fit, 19687, true);
    let armor = sol_sys.get_item_attrs(&ship.id).unwrap().get(&265).unwrap().dogma;
    println!("{armor}");
    let rig = sol_sys.add_rig(fit, 31057, true).unwrap();
    let armor = sol_sys.get_item_attrs(&ship.id).unwrap().get(&265).unwrap().dogma;
    println!("{armor}");
    sol_sys.remove_item(&rig.id);
    let armor = sol_sys.get_item_attrs(&ship.id).unwrap().get(&265).unwrap().dogma;
    println!("{armor}");
}
