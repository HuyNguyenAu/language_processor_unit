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
        return match self.data.get(address) {
            Some(bytes) => Ok(bytes),
            None => Err(format!("Address out of bounds: {}", address)),
        };
    }

    pub fn length(&self) -> usize {
        return self.data.len();
    }
}
