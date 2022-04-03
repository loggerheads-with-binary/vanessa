extern crate lazy_static;

use crate::chaeyoung;

lazy_static::lazy_static!{

    pub static ref  media_dir : String  = {

        let content = "{Lois}/Toolkit/Extra/Notification/notif.rust/media";

        let answer = chaeyoung::chaeyoung_translate(content);

        let answer = answer.expect("Chaeyoung Translate Failed").to_string();

        answer
    };

    pub static ref  log_file : String = {

        let content = "{Lois}/Toolkit/Extra/Notification/notif.rust/standard_log.csv";

        let answer = chaeyoung::chaeyoung_translate(content);

        let answer = answer.expect("Chaeyoung Translate Failed").to_string();

        answer
    };


    pub static ref default_icon : String = {

        let content = "{Lois}/Toolkit/Extra/Notification/notif.rust/media/Blackpink-754368.png";

        let answer = chaeyoung::chaeyoung_translate(content);

        let answer  = answer.expect("Chaeyoung Translate Failed").to_string();

        answer

    };

}
