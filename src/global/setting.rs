pub struct Setting {
    pub auto_discover: bool,
}

impl Setting {
    pub fn new() -> Self {
        Self {
            auto_discover: true,
        }
    }
}
