extern crate rustc_serialize as serialize;

use set1::serialize::hex::FromHex;
use set1::serialize::hex::ToHex;
use set1::serialize::base64::ToBase64;

pub fn hex_to_64(input: String) -> String{
    let output = input.from_hex().unwrap().to_base64(serialize::base64::STANDARD);
    output
}

pub fn xor(first: String, second: String) -> String{
    let first_h = first.from_hex().unwrap();
    let second_h = second.from_hex().unwrap();
    let mut xor_result: Vec<u8> = Vec::new();
    for (elem_of_first, elem_of_second) in first_h.iter().zip(second_h.iter()) {
        xor_result.push(elem_of_first ^ elem_of_second)
    }
    xor_result.to_hex()
}