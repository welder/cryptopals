use std::env;
mod set1;

fn main() {
    let input = env::args().nth(1).unwrap();
    let output = set1::hex_to_64(input);
    print!("{}", output);

    println!("Now demonstrating XOR!");
    println!("1c0111001f010100061a024b53535009181c XOR 686974207468652062756c6c277320657965");

    let first   = String::from("1c0111001f010100061a024b53535009181c");
    let second  = String::from("686974207468652062756c6c277320657965");

    let xor_answer = set1::xor(first, second);

    println!("produces {}", xor_answer);
}