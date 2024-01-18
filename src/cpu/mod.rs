use std::fs;

mod register;

const CPU_BITS: usize = 16;

pub fn load_and_encode_instsructions(file: &str) {
    let content = match fs::read(file) {
        Ok(c) => c,
        Err(e) => {
            println!("Error reading file: {:?}", e);
            return;
        }
    };

    if content.len() % CPU_BITS != 0 {
        println!("Invlaid input size");
        return;
    }
}
