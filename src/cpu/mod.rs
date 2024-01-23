use crate::cpu::command::Command;
use std::fs;

mod command;
mod storage;

const CPU_BITS: usize = 16;

pub fn load_and_encode_instsructions(file: &str) {
    let content = match fs::read(file) {
        Ok(c) => c,
        Err(e) => {
            println!("Error reading file: {:?}", e);
            return;
        }
    };

    let array_len_per_cmd = CPU_BITS / 8;

    if content.len() % array_len_per_cmd != 0 {
        println!("Invalid input size");
        return;
    }

    for cmd in Command::from(&content) {
        println!("{}", cmd);
    }
}
