pub struct MemoryUnit {
    data: Vec<u8>,
}

impl MemoryUnit {
    pub fn new() -> Self {
        MemoryUnit { data: Vec::new() }
    }

    pub fn load(&mut self, bytecode: Vec<u8>) {
        println!("Loading bytecode of length {}", bytecode.len());

        self.data = bytecode;
    }

    pub fn read_byte(&self, address: &u8) -> &u8 {
        return match self.data.get(*address as usize) {
            Some(byte) => byte,
            None => panic!("Address out of bounds."),
        };
    }

    pub fn data_length(&self) -> usize {
        self.data.len()
    }
}