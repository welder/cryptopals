use std::env;
mod set1;

fn main() {
    let input = env::args().nth(1).unwrap();
    let output = set1::hex_to_64(input);
    print!("{}", output);
}