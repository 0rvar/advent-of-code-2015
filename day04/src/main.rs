use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fmt::Write;

fn main() {
    let input = "bgvyzdsv".as_bytes();

    let mut hasher = Md5::new();
    let mut part_1_found = false;

    for i in 0..std::u64::MAX {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);
        let hash = to_hex(&output);

        if !part_1_found && hash.starts_with("00000") {
            println!("Part 1: {}", i);
            part_1_found = true
        }
        if hash.starts_with("000000") {
            println!("Part 2: {}", i);
            break;
        }
        hasher.reset();
    }
}

fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &byte in bytes {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    s
}
