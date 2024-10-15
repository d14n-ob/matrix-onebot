use matrix_sdk::ruma::MilliSecondsSinceUnixEpoch;
use walle_core::prelude::Selft;
use crate::constant::PLATFORM;

pub fn get_self() -> Selft {
    Selft {
        platform: PLATFORM.to_owned(),
        user_id: "等实现config之后从config里读，不想传client参".to_string(),
    }
}

pub fn get_time(ts: MilliSecondsSinceUnixEpoch) -> f64 {
    let num = ts.get().to_string().parse::<f64>();

    if let Ok(num) = num {
        num / 1000f64
    } else {
        0f64
    }

}