pub struct MemoryUnit {
    data: Vec<u8>,
}

impl MemoryUnit {
    pub fn new() -> Self {
        MemoryUnit { data: Vec::new() }
    }

    pub fn load(&mut self, bytecode: Vec<u8>) {
        self.data = bytecode;
    }

    pub fn read(&self, address: usize) -> [u8; 4] {
        let bytes = match self.data.get(address..address + 4) {
            Some(bytes) => bytes,
            None => panic!("Address out of bounds."),
        };

        return [bytes[0], bytes[1], bytes[2], bytes[3]];
    }

    pub fn length(&self) -> usize {
        return self.data.len();
    }
}
