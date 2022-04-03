extern crate notify_rust;
use self::notify_rust::{Notification,Hint,Timeout};

pub fn standard_notification(prog : &str , title : &str , message : &str , icon : &str , duration : i32) -> () {

    let result = Notification::new()
                .summary(title)
                .appname(prog)
                .body(message)
                .icon(icon)
                .timeout(Timeout::Milliseconds((1000*duration as u32))).show().expect("Vanessa Notification Failed");


    return ;

}

pub fn persist_notification(prog : &str , title : &str , message : &str , icon : &str , duration : i32) -> () {


    let result = Notification::new()
                .summary(title)
                .appname(prog)
                .hint(Hint::Resident(true))
                .icon(icon)
                .body(message)
                .timeout(Timeout::Never)
                .show().expect("Vanessa Notification Failed");

    return ;

}
