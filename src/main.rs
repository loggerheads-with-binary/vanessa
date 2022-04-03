#![windows_subsystem = "windows"]
//#[target_os = "windows"]
use std::process::Command;

mod chaeyoung;
mod filepaths;

#[cfg(target_os = "windows")]
mod windows_vanessa;

#[cfg(target_os = "windows")]
use windows_vanessa::{
    persist_notification, standard_notification
};

#[cfg(not (target_os = "windows"))]
mod linux_vanessa;

#[cfg(not (target_os = "windows"))]
use linux_vanessa::{
    persist_notification , standard_notification
};

use filepaths::{media_dir , log_file , default_icon};



//Arguments
extern crate clap;
use clap::{load_yaml, App};

use std::fs::OpenOptions;
//use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

extern crate chrono;
extern crate rand;
use rand::{OsRng, Rng};
use std::fs;
use std::io::Write;

extern crate glob;
use glob::{glob, GlobError};
use std::convert::TryInto;
use std::path::PathBuf;

extern crate json;

#[macro_use(c)]
extern crate cute;

#[cfg(debug_assertions)]
extern crate log;

#[cfg(debug_assertions)]
extern crate simple_logger;

fn get_random_file() -> String {
    let paths = c![x, for x in fs::read_dir(std::path::Path::new(&*media_dir)).expect("Failed Directory List Read") ];

    let n: u32 = paths.len().try_into().unwrap();

    let mut rng = match OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS RNG: {}", e),
    };

    let num: u32 = rng.next_u32();

    let x = paths[(num % n) as usize].as_ref().unwrap();
    let y = x
        .path()
        .into_os_string()
        .into_string()
        .unwrap_or(default_icon.to_string());

    #[cfg(debug_assertions)]
    log::debug!("Random File Chosen:: {}", y);

    y
}

fn make_notif(
    title: &str,
    message: &str,
    icon: &str,
    prog: &str,
    wait_period: Duration,
    duration: i32,
    reminder_sleep: Duration,
    snooze_count: usize,
    persist: bool,
) -> () {
    #[cfg(debug_assertions)]
    log::debug!(
        "{prog} calling with {title} title and {message} body",
        prog = prog,
        title = title,
        message = message
    );

    #[cfg(debug_assertions)]
    log::debug!("Sleeping for {:#?} ", wait_period);

    sleep(wait_period);

    for i in 0..snooze_count {
        #[cfg(debug_assertions)]
        log::debug!("Making Standard Notification");

        standard_notification(prog, title, message, icon, duration);
        sleep(reminder_sleep);
    }

    if persist {
        log::debug!("Making Final Standard Notification [Persistent]");
        persist_notification(prog, title, message, icon, duration);
    } else {
        #[cfg(debug_assertions)]
        log::debug!("Making Final Standard Notification [Non-Persistent]");

        standard_notification(prog, title, message, icon, duration);
    }
}

fn log_notif(title: &str, message: &str, prog: &str, snooze_count: usize, duration: i32) -> () {
    //For Windows only
    let date_ = chrono::offset::Local::now()
        .format("%H:%M:%S on %d-%m-%Y")
        .to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&*log_file)
        .unwrap();

    writeln!(
        file,
        "\"{date_}\",\"{prog}\",\"{title}\",\"{message}\",{duration},{snooze_count}",
        date_ = date_,
        title = title.escape_default(),
        prog = prog.escape_default(),
        message = message.escape_default(),
        duration = duration,
        snooze_count = snooze_count
    );

    return;
}

fn main() -> () {
    #[cfg(debug_assertions)]
    simple_logger::SimpleLogger::new().init().unwrap();

    let yaml = load_yaml!(".././clap.yaml"); //Read from local directory for cargo build
    let matches = App::from_yaml(yaml).get_matches();

    let prog = matches.value_of("prog").unwrap_or("Anonymous Program");
    let title = matches.value_of("title").unwrap_or("Standard Notification");
    let message = matches
        .value_of("message")
        .unwrap_or("A Message from an anonymous program");
    let mut icon: String = matches.value_of("icon").unwrap_or("").to_string();
    let wait_period = Duration::from_millis(
        matches
            .value_of("wait_period")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0)
            * 1000,
    );
    let reminder_sleep = Duration::from_millis(
        matches
            .value_of("reminder_sleep")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0)
            * 1000,
    );
    let duration: i32 = matches
        .value_of("duration")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(5);
    let snooze_count: usize = matches
        .value_of("snooze_count")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    let persist: bool = matches.is_present("persist");

    if icon == "" {
        icon = get_random_file();
    }


    make_notif(
        title,
        message,
        &icon,
        prog,
        wait_period,
        duration,
        reminder_sleep,
        snooze_count,
        persist,
    );

    log_notif(title, message, prog, snooze_count, duration);

    return;
}
