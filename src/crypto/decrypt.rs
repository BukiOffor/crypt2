#![allow(unused_imports,deprecated)]
use simple_crypt::{
    decrypt as simple_decrypt,
    decrypt_file as simple_decrypt_file
};
use std::{path::Path, process::exit};

/// Decrypts a string using the given password.
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

/// Decrypts a file using the given password.
pub fn decrypt_file(file: String, password: &str) -> Result<(),Box<dyn std::error::Error>> {
    let path = Path::new(&file);
    if !path.exists() {
        eprintln!("Error: File does not exist");
        exit(1);
    }
    let response = simple_decrypt_file(path, path, password.as_bytes()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1)
    });
    Ok(response)
}