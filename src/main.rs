use std::fs::File;
use std::io::Read;

use chip8::Chip8;

mod ram;
mod chip8;


fn main () { 
    let mut file = File::open("data/TETRIS").unwrap();
    let mut data = Vec::<u8>::new();
    match file.read_to_end(&mut data)  {
        Ok(_) => {println!("Reading rom file. . ."); },
        Err(_) => println!("There was an error reading the ROM file."),
    };
    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);
    
}
