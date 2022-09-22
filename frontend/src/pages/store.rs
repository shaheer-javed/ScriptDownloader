use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct YewduxStore {
    pub option: String,
    pub textarea: String,
}

impl Persistent for YewduxStore {
    fn key() -> &'static str {
        "IntroductionToYew.rs"
    }

    fn area() -> Area {
        Area::Local
    }
}