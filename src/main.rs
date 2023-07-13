mod ihex_mgmt;
mod file_mgmt;

use std::io;
use std::io::Write;
//use std::fs;

//use std::{thread, time};
//use std::time::Duration;

use env_logger::{Builder, Target};
use log::LevelFilter;



//use ihex_mgmt::Segment;
use crate::ihex_mgmt::ihex_mgmt::Segment;

use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;

fn main() -> io::Result<()> {
    println!("whatavr starting ...");

    init_logging();
    log_start();

    //
    // load hex file
    //

    let mut hex_file_path:String = String::new();
    //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank1.hex");
    //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank2.hex") {
    hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication1/GccApplication1.hex");

    // split into segments
    // each segment has to have a segment_start and a segment_size
    let mut segments: Vec<Segment> = Vec::new();
    match parse_hex_file(&mut segments, &hex_file_path) {
        Ok(_name) => log::info!("File read"),
        Err(err) => {
            log::error!("An error occured while retrieving the peername: {:?}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error at load hex file!"));
        }
    }

    // // DEBUG dump parsed segments
    // for seg in segments.iter_mut() {
    //     log::info!("Segment: {}", seg);
    // }

    // process the first segment only
    let ref segment_0 = &segments[0];
    log::info!("Segment: {}", segment_0);

    log_end();

    Ok(())
}

fn init_logging() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        // https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn log_start() {
    log::trace!("Application starts ...");
    log::debug!("Application starts ...");
    log::info!("Application starts ...");
    log::warn!("Application starts ...");
    log::error!("Application starts ...");
}

fn log_end() {
    log::trace!("Application terminates.");
    log::debug!("Application terminates.");
    log::info!("Application terminates.");
    log::warn!("Application terminates.");
    log::error!("Application terminates.");
}
