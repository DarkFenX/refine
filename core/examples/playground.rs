#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::PathBuf;

use chrono;

use reefast::{
    cg,
    dh::{self, DataHandler},
    dh_impls::phb_file,
    ch_impls::json_file,
};

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

fn print_data<T>(name: &'static str, data: dh::Result<dh::Container<T>>) {
    match data {
        Ok(r) => {
            println!("{}: {} returned, {} failed", name, r.data.len(), r.warns.len());
            for e in r.warns.iter() {
                println!("  error: {}", e)
            }
        }
        Err(e) => println!("{} failed: {}", name, e),
    }
}

fn main() {
    setup_logger().unwrap();
    let dh = phb_file::PhbFileDHandler::new(PathBuf::from("/home/dfx/Desktop/phobos_tq_en-us"));
    let cont = cg::generate_cache(&dh).unwrap();
    let ch = json_file::JsonFileCHandler::new(PathBuf::from("/home/dfx/Workspace/eve/reefast/cache/tq.json.bz2"));
}
