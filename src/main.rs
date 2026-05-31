use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: aes-crypt <encrypt|decrypt> <file> <key>");
        return;
    }

    let mode = &args[1];
    let filename = &args[2];
    let key = args[3].as_bytes();

    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Unable to read file");

    let output = if mode == "encrypt" {
        xor_cipher(&contents, key)
    } else {
        xor_cipher(&contents, key)
    };

    let mut out_file = File::create(filename).expect("Unable to write file");
    out_file.write_all(&output).expect("Encryption write error");
    println!("[+] Operation completed successfully!");
}

fn xor_cipher(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}