use std::env;
mod set1;

fn main() {
    //first exercise: grab input and convert it from hex to base64
    let input = env::args().nth(1).unwrap();
    let output = set1::hex_to_64(input);
    println!("{}", output);
    println!("");

    //second exercise: given two equal length buffers (provided), return xor
    println!("Now demonstrating XOR!");

    let first = String::from("1c0111001f010100061a024b53535009181c");
    let second = String::from("686974207468652062756c6c277320657965");

    let xor_answer = set1::xor(first, second);

    print!("{}", xor_answer);
}