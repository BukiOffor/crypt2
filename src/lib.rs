use bip39::*;
use std::error::Error;


///This function generates a word phrase with the specific word count passed as an arguement
pub(crate) fn get_mnemonic(count:u8) -> Result<String, dyn Error> {
    let mut rng = rand::thread_rng();
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, count).
}
