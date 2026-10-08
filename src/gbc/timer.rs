#[derive(Default)]
pub struct Timer {
    value: u32,
}

impl Timer {
    pub fn tick(&mut self, by: u32) { self.value = self.value.saturating_sub(by); }

    pub fn value(&self) -> u32 { self.value }

    pub fn set(&mut self, value: u32) { self.value = value; }
}
