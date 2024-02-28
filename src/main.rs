use log::{error, info};
use simple_logger::SimpleLogger;

pub mod leo_to_moon;
pub mod multi_planet;
pub mod single_planet;
pub mod tracer;

fn main() {
    SimpleLogger::new().init().unwrap();

    let Some(arg) = std::env::args().nth(1) else {
        error!("missing argument. Usage: cargo run (--release) <simulation>");
        return;
    };

    info!("running simulation `{arg}`");

    match arg.as_str() {
        "single_planet" => single_planet::start(),
        "multi_planet" => multi_planet::start(),
        "leo_to_moon" => leo_to_moon::start(),
        _ => panic!("unknown argument"),
    }
}
