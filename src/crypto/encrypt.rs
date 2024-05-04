#![allow(deprecated)]

use simple_crypt::encrypt as simple_encrypt;
use std::process::exit;


pub fn encrypt(text: &str, password: &str) -> Result<String,Box<dyn std::error::Error>> {
    let text = text.as_bytes();
    let password = password.as_bytes();
    let cipher_in_bytes = simple_encrypt(text, password).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });
    let cipher = base64::encode(&cipher_in_bytes);
    Ok(cipher)
}