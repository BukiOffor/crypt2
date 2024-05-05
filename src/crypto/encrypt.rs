#![allow(deprecated)]

use simple_crypt::{
    encrypt as simple_encrypt,
    encrypt_file as simple_encrypt_file,
    encrypt_directory as simple_encrypt_directory,
};
use std::process::exit;
use std::path::Path;


/// Encrypt a string with a given password.
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

/// Encrypt a file with a given password.
pub fn encrypt_file(file: String, password: &str) -> Result<(),Box<dyn std::error::Error>> {
    let path = Path::new(&file);
    if !path.exists() {
        eprintln!("Error: File does not exist");
        exit(1);
    }
    let response = simple_encrypt_file(path, path, password.as_bytes()).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1)
    });
    Ok(response)
}

pub fn encrypt_dir(dir: &str, password: &str) -> Result<(),Box<dyn std::error::Error>> {
    let dir = Path::new(dir);
    if !dir.exists() {
        eprintln!("Error: Directory does not exist");
        exit(1);
    }
    let response = simple_encrypt_directory(dir, Path::new("example.dir"), password.as_bytes())
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            exit(1)
        });
    Ok(response)
}