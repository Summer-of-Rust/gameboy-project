pub struct Ppu {}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {}
    }

    pub fn tick(&mut self) {
        log::debug!("Ppu tick!");
    }
}
