mod java_glue;

pub use crate::java_glue::*;

use android_logger::Config;
use log::LevelFilter;
use rifgen::rifgen_attr::*;

pub struct RustLog;

impl RustLog {
    //set up logging
    #[generate_interface]
    pub fn initialise_logging() {
        #[cfg(target_os = "android")]
            android_logger::init_once(
            Config::default()
                .with_max_level(LevelFilter::Trace)
                .with_tag("Rust"),
        );
        log_panics::init();
        log::info!("Logging initialised from Rust");
    }
}

pub struct Inputs {
    first: i64,
    second: i64,
}

impl Inputs {
    #[generate_interface(constructor)]
    pub fn new(first: i64, second: i64) -> Inputs {
        Self {
            first,
            second,
        }
    }
    #[generate_interface]
    pub fn addition(&self) -> i64 {
        log::info!("Adding the value");
        self.first + self.second
    }
    #[generate_interface]
    pub fn subtraction(&self) -> i64 {
        log::info!("subtraction the value");
        self.first - self.second
    }
    #[generate_interface]
    pub fn multiplication(&self) -> i64 {
        log::info!("multiplication the value");
        self.first * self.second
    }

    #[generate_interface]
    pub fn make_call(&self) -> Result<String, String> {
        reqwest::blocking::get("https://pastebin.com/raw/kRMwpCJS").unwrap()
            .text().map_err(|e| e.status().unwrap().as_str().to_owned())
    }
}