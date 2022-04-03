

//
// pub const media_dir: &'static str = r"D:/Toolkit/Extra/Notification/notif.rust/media/";
// pub const log_file: &'static str = r"D:/Toolkit/Extra/Notification/notif.rust/standard_log.csv";
// pub const default_icon: &'static str =
//     r"D:/Toolkit/Extra/Notification/notif.rust/media/Blackpink-754368.png";

extern crate winrt_notification;
use self::winrt_notification::{Duration, IconCrop, Sound, Toast};

extern crate log;
use std::path::Path;

static mut ONE_FAIL: bool = false;
//pub static APPMODELUSERID: &'static str =  Toast::POWERSHELL_APP_ID;
pub static APPMODELUSERID : &'static str = "Vanessa.Notification.v.2.0.0" ; 

pub fn standard_notification(
    prog: &str,
    title: &str,
    message: &str,
    icon: &str,
    duration: i32,
) -> () {
    #[cfg(debug_assertions)]
    log::debug!("Standard Call Received for {b} seconds:: {a} calls of Short 7 second notifications will be made"  , b= duration , a = 1 + duration/7);

    for i in 0..=(duration / 7) {
        #[cfg(debug_assertions)]
        log::info!("\tMaking Notification Toast");

        let toast = Toast::new(APPMODELUSERID)
            .title(title)
            //.text1(title)
            .text2(message)
            .icon(Path::new(icon), IconCrop::Square, "Icon Load Failed")
            .duration(Duration::Short)
            .show();

        if toast.is_err() {
            unsafe {
                if ONE_FAIL {
                    #[cfg(debug_assertions)]
                    log::error!("Notification Call has Failed");
                    std::process::exit(0xff);
                }

                ONE_FAIL = true;
            }

            //standard_notification(prog, title, message, default_icon, duration);
            return;
        }
    }
}

pub fn persist_notification(
    prog: &str,
    title: &str,
    message: &str,
    icon: &str,
    duration: i32,
) -> () {
    //No way to persist COM notification on Windows using RUST

    standard_notification(prog, title, message, icon, duration);
}
