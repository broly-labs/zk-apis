#![no_main]

sp1_zkvm::entrypoint!(main);

pub fn main() {
    let input = sp1_zkvm::io::read::<String>();
    let output = sp1_zkvm::io::read::<String>();

    if output != input {
        panic!("Input does not match output");
    }
}