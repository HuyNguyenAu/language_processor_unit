pub struct MemoryUnit {
    data: Vec<[u8; 4]>,
}

impl MemoryUnit {
    pub fn new() -> Self {
        MemoryUnit { data: Vec::new() }
    }

    pub fn load(&mut self, byte_code: Vec<[u8; 4]>) {
        self.data = byte_code;
    }

    pub fn read(&self, address: usize) -> &[u8; 4] {
        return match self.data.get(address) {
            Some(bytes) => bytes,
            None => panic!("Address out of bounds."),
        };
    }

    pub fn length(&self) -> usize {
        return self.data.len();
    }
}
