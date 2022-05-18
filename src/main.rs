#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::*;
use std::fs::File;
use std::env;
use std::path::Path;
use std::process::Command;
use std::{thread, time::Duration};

fn kill_process(name: &str) {
  let output = Command::new("killall")
    .arg(name)
    .output()
    .expect("failed to execute process");
  info!("{}", String::from_utf8_lossy(&output.stdout));
  info!("{}", String::from_utf8_lossy(&output.stderr));
}

fn open_app(name: &str) {
  let full_app_name = format!("{}.app", name);
  let output = Command::new("open")
    .arg("-a")
    .arg(full_app_name)
    .output()
    .expect("failed to execute process");
  info!("{}", String::from_utf8_lossy(&output.stdout));
  info!("{}", String::from_utf8_lossy(&output.stderr));
}

fn hpatchz_app(hpatchz_path: &str, delta_path: &str, app_name: &str) {
  let path = Path::new(hpatchz_path);
  let app_path = format!("/Applications/{}.app", app_name);

  let output = Command::new(path)
    .arg("-C-all")
    .arg(&app_path)
    .arg(delta_path)
    .arg(&app_path)
    .arg("-f")
    .output()
    .expect("failed to execute hpatchz process");

  info!("{}", String::from_utf8_lossy(&output.stdout));
  info!("{}", String::from_utf8_lossy(&output.stderr));
}

fn help() {
  info!("Usage: mac-updater <app-name> <delta-path> <hpatchz-path>");
}

fn main() {

  CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("mac_updater.log").unwrap()),
        ]
    ).unwrap();

  let args: Vec<String> = env::args().collect();

  match args.len() {
    1..=3 => help(),
    4 => {
      let app_name = &args[1];
      let delta_path = &args[2];
      let hpatchz_path = &args[3];

      kill_process(app_name);
      hpatchz_app(hpatchz_path, delta_path, app_name);
      thread::sleep(Duration::from_secs(1));
      open_app(app_name);
    }
    _ => help(),
  }
}
