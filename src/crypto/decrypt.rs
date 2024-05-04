#![allow(unused_imports,deprecated)]
use simple_crypt::decrypt as simple_decrypt;
use std::process::exit;


pub fn decrypt(text: &str, password: &str) -> Result<String,Box<dyn std::error::Error>> {
    let text = base64::decode(text.to_owned())?;
    let password = password.as_bytes();
    let cipher_in_bytes = simple_decrypt(&text, password).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });
    let cipher = String::from_utf8(cipher_in_bytes)?;
    Ok(cipher)
}