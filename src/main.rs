#![windows_subsystem = "windows"]

use std::error::Error;

#[cfg(feature = "fuzzy-matcher")]
mod fuzzy;
#[cfg(feature = "backend")]
mod model;
mod rfd_service;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("ModSwapper", native_options, ui::app()).map_err(|a| a.into())
}
