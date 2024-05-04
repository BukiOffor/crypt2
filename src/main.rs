use crypt2::{get_mnemonic,crypto::{encrypt::encrypt, decrypt::decrypt}};

fn main() {
    // lock a file with a password
    let res = get_mnemonic(18).unwrap();
    println!("phrase: {}", res);
    let enc_string = encrypt(&res, "p").unwrap();
    println!("encrypted phrase: {}", enc_string);
    let dec_string = decrypt(&enc_string, "p").unwrap();
    println!("decrypted string: {}", dec_string);

}

// TO DO 
    // lock a file with a password
    // decrypt a file with a password
    // encrypt a file with a password
