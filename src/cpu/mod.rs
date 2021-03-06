#[allow(dead_code)]
pub struct Cpu {
    running: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { running: false }
    }

    pub fn tick(&mut self) {
        log::debug!("Cpu tick!");
    }
}
