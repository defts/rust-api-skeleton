extern crate clap;
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate iron;

use clap::{Arg, App};
use env_logger::Builder;
use iron::{Iron};
use log::info;
mod routes;
use routes::routes::app_router;
use std::error::Error;

mod handlers;
mod storage;

#[derive(Debug)]
struct Config {
    log_level: String,
    log_format: String,
    port: i16,
}

impl Config {
    fn new(args: clap::ArgMatches<'_>) -> Result<Config, Box<dyn Error>> {
        let log_level = args.value_of("log_level").unwrap();
        let log_format = args.value_of("log_format").unwrap();
        let port = args.value_of("port").unwrap();

        let log_format = String::from(log_format);
        let port: i16 = port.parse()?;

        Builder::new().parse_filters(log_level).init();
        
        let log_level = String::from(log_level);
        Ok(Config { log_level, log_format, port })
    }
}

fn main() {
    let matches = App::new("Rust API Skeleton")
        .version("0.0.1")
        .author("David Torres Dias <david.torresdias@gmail.com>")
        .about("Rust API Skeleton")
        .arg(Arg::with_name("log_level")
                 .long("log-level")
                 .takes_value(true)
                 .default_value("warning")
                 .help("Use this flag to set the logging level"))
        .arg(Arg::with_name("log_format")
                 .long("log-format")
                 .takes_value(true)
                 .default_value("text")
                 .help("Use this flag to set the logging format"))
        .arg(Arg::with_name("port")
                 .long("port")
                 .takes_value(true)
                 .default_value("8080")
                 .help("Use this flag to set the listening port of the api"))
        .get_matches();

        let config = match Config::new(matches) {
            Ok(config) => config,
            Err(e) => {
                panic!("There was a problem: {:?}", e)
            }
        };


    let server_url:String = format!("0.0.0.0:{}", config.port);

    info!("server is running at {}", server_url);

    Iron::new(app_router()).http(server_url).unwrap();
} 