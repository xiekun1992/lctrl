use serde::{Deserialize, Serialize};

use crate::web_api::dto::ScreenSetting;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub auto_discover: bool,
    pub screen_setting: ScreenSetting,
}

impl Setting {
    pub fn new() -> Self {
        Self {
            auto_discover: true,
            screen_setting: ScreenSetting::new(),
        }
    }
}
