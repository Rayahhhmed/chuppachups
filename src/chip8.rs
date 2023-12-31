use ram::Ram;

use crate::ram;

pub struct Chip8 { 
  ram: Ram
}

impl Chip8 {
  pub fn new() -> Self { 
    Self { 
      ram: Ram::new()
    }
  }

  pub fn load_rom(&mut self, data: &Vec<u8>) {
    let off = 0x200;
    for idx in 0..data.len() {
      self.ram.write_byte((off + idx) as u16, data[idx]);
    }
  }

  
}