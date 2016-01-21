extern crate rustc_serialize as serialize;

use set1::serialize::hex::FromHex;
use set1::serialize::base64::ToBase64;

pub fn hex_to_64(input: String) -> String{
    let output = input.from_hex().unwrap().to_base64(serialize::base64::STANDARD);
    output
}