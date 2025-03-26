use super::windows;


pub enum System {
    Windows,
}

pub struct Platform {
    pub system: System,
}

impl Platform {
    pub fn install(&self) {
        match self.system {
            System::Windows => windows::install(),
        }
    }
}
