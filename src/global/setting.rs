use serde::{Deserialize, Serialize};

use crate::web_api::dto::ScreenSetting;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseWheelStyle {
    Traditional = 0, // 滚轮控制滚动条滚动
    Natural = 1,     // 滚轮控制内容滚动
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub auto_discover: bool,
    pub screen_setting: ScreenSetting,
    pub cursor_across_screens: bool,
    pub scale_factor: f32,
    pub mouse_wheel_style: MouseWheelStyle,
    pub enable_control: bool,
}

impl Setting {
    pub fn default() -> Self {
        Self {
            auto_discover: true,
            cursor_across_screens: true, // 是否允许光标跨屏控制和解控
            scale_factor: 1.0,           // 鼠标移动的缩放因子
            mouse_wheel_style: MouseWheelStyle::Traditional,
            screen_setting: ScreenSetting::new(),
            enable_control: true, // 是否允许控制远程设备
        }
    }
}
