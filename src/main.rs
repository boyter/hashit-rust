// https://github.com/RustCrypto/hashes

use md5::{Md5, Digest};
use clap::{App, Arg};
use std::fs::File;
use std::io::Read;
use sha1::Sha1;
use sha2::Sha256;
use blake2::{Blake2b, Blake2s};

fn main() {
    let matches = App::new("hashit")
        .version("1.0")
        .about("Hash files in many hashing algorithms")
        .author("Ben Boyter")
        .arg(Arg::with_name("FILES")
            .help("Sets the input file(s) to hash")
            .required(true)
            .multiple(true)
            .index(1))
        .get_matches();


    // Safe to unwrap as INPUT is a required filed
    let dir_names: Vec<&str> = matches.values_of("FILES").unwrap().collect();

    for (_size, &s) in dir_names.iter().enumerate() {
        println!("{}", s);
        hash_file(s);
        println!("");
    }
}

fn hash_file(filename: &str) {
    // For the first attempt just read the whole file then call out to hashing functions
    if let Ok(mut x) = File::open(filename) {
        let mut buffer = Vec::with_capacity(x.metadata().unwrap().len() as usize);
        x.read_to_end(&mut buffer);

        hash_md5(&buffer);
        hash_sha1(&buffer);
        hash_sha256(&buffer);
        hash_blake2b(&buffer);
        hash_blake2s(&buffer);
    }
}

fn hash_md5(content: &Vec<u8>) {
    let mut hasher = Md5::new();
    hasher.input(content);
    println!("MD5 {:x}", hasher.result());
}

fn hash_sha1(content: &Vec<u8>) {
    let mut hasher = Sha1::new();
    hasher.input(content);
    println!("SHA1 {:x}", hasher.result());
}

fn hash_sha256(content: &Vec<u8>) {
    let mut hasher = Sha256::new();
    hasher.input(content);
    println!("SHA256 {:x}", hasher.result());
}

fn hash_blake2b(content: &Vec<u8>) {
    let mut hasher = Blake2b::new();
    hasher.input(content);
    println!("BLAKE2 {:x}", hasher.result());
}

fn hash_blake2s(content: &Vec<u8>) {
    let mut hasher = Blake2s::new();
    hasher.input(content);
    println!("BLAKE2s {:x}", hasher.result());
}