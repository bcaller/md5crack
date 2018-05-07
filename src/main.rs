extern crate crypto;

use std::env;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Need a hash as a target")
    }
    let target = &args[1];
    if target.chars().count() != 32 {
        panic!("Hash should be 32 chars long")
    }
    let mut target_bytes = [0; 16];
    for i in 0..16 {
        target_bytes[i] = u8::from_str_radix(&target[2*i..(2*i+2)], 16).unwrap_or(0);
    }
    println!("Searching for {}", target);

    let mut md5 = Md5::new();
    for i in (0..100000000).rev() {
        let key = format!("{:08}", i);
        md5.input_str(&key);
        let mut output_bytes = [0; 16];
        md5.result(&mut output_bytes);
        if i % 1000000 == 0 {
            println!("{} - {}", key, md5.result_str())
        }
        if output_bytes == target_bytes {
            println!("{} - {}", key, md5.result_str());
            break
        }
        md5.reset()
    }
}
