pub mod utils;
pub mod crypto;
use bip39::*;
use utils::errors::Crypt2Errors;
use std::error::Error;


fn get_word_count(count:u8) -> Result<MnemonicType, impl Error> {
    match count {
        12 => Ok(MnemonicType::Words12),
        15 => Ok(MnemonicType::Words15),
        18 => Ok(MnemonicType::Words18),
        21 => Ok(MnemonicType::Words21),
        24 => Ok(MnemonicType::Words24),
        _ => Err(Crypt2Errors::WordCountCouldNotBeParsed(count))
    }
}

///This function generates a word phrase with the specific word count passed as an arguement. 
/// It throws an error if the word count passed is not valid

///```rust
/// use crypt2::get_mnemonic;
/// fn get_12_word_mnemonic() {
///     let mnemonic: String = get_mnemonic(12).unwrap();
///     let phrase_vec: Vec<&str> = mnemonic.split_whitespace().into_iter().collect::<Vec<&str>>();
///     assert_eq!(phrase_vec.len(), 12);
///  } 
/// ```


pub fn get_mnemonic(count:u8) -> Result<String, Box<dyn Error>> {
    let count = get_word_count(count)?;
    let mnemonic = Mnemonic::new(count, Language::English);
    let mnemonic = mnemonic.to_string();
    Ok(mnemonic)
}


pub fn get_string_from_bytes(bytes_data: &[u8]) -> String {
    let mut string_data = String::new();
    for byte in bytes_data {
        string_data.push(*byte as char);
    }
    string_data
}

pub fn get_bytes_from_string(string_data: String) -> Vec<u8> {
    let mut bytes_data = Vec::new();
    for ch in string_data.chars() {
        bytes_data.push(ch as u8);
    }
    bytes_data
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_12_word_mnemonic() {
        let mnemonic = get_mnemonic(12).unwrap();
        let mnemonic = mnemonic.split_whitespace().into_iter().collect::<Vec<&str>>();
        assert_eq!(mnemonic.len(), 12);
    }

    #[test]
    fn test_get_15_word_mnemonic(){
        let mnemonic = get_mnemonic(15).unwrap();
        let mnemonic = mnemonic.split_whitespace().into_iter().collect::<Vec<&str>>();
        assert_eq!(mnemonic.len(), 15);
    }
    #[test]
    fn test_get_24_word_mnemonic(){
        let mnemonic = get_mnemonic(24).unwrap();
        let mnemonic = mnemonic.split_whitespace().into_iter().collect::<Vec<&str>>();
        assert_eq!(mnemonic.len(), 24);
    }
    #[test]
    fn test_fails_if_wrong_number_is_passed(){
        let mnemonic = get_mnemonic(30);
        assert!(mnemonic.is_err());
    }
}