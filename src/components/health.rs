pub struct Health {
    pub current: u32,
    pub maximum: u32,
}

impl Health {
    pub fn new(max_health: u32) -> Self {
        Self {
            current: max_health,
            maximum: max_health,
        }
    }
}
