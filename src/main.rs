#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sudoku;
mod ui;

use ui::app::App;
use ui::types::AppInput;
use windows_reactor::App as ReactorApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ReactorApp::run_component::<App>(AppInput)?;
    Ok(())
}
