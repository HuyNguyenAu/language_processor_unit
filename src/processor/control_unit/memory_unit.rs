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

    pub fn read(&self, address: usize) -> Result<&[u8; 4], String> {
        self.data
            .get(address)
            .ok_or_else(|| format!("Address out of bounds: {}", address))
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}
