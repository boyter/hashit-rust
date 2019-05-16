// https://github.com/RustCrypto/hashes

use md5::{Md5, Digest};
use clap::{App, Arg};

fn main() {
    let _matches = App::new("hashit")
        .version("1.0")
        .about("Hash files in many hashing algorithms")
        .author("Ben Boyter")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file(s) to hash")
            .required(true)
            .multiple(true)
            .index(1))
        .get_matches();

    let mut hasher = Md5::new();
    let data = b"Hello world!";
    hasher.input(data);
    hasher.input("String data");
    let hash = hasher.result();
    println!("Result: {:x}", hash);
}
