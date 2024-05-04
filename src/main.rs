use crypt2::get_mnemonic;

fn main() {
    let res = get_mnemonic(12).unwrap();
    println!("{}", res);
}
