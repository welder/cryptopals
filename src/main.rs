use std::env;
mod set1;

fn main() {
    let input = env::args().nth(1).unwrap();
    let output = set1::hex_to_64(input);
    print!("{}", output);

    println!("Now demonstrating XOR!");

    let first = 6;
    let second = 10;

    let xor_answer = first ^ second;

    println!("produces {}", xor_answer);
}