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
    println!("Searching for {}", target);

    let mut md5 = Md5::new();
    for i in (0..100000000).rev() {
        let key = format!("{:08}", i);
        md5.input_str(&key);
        let output = md5.result_str();
        if i % 1000000 == 0 {
            println!("{} - {}", key, output)
        }
        if output.eq(target) {
            println!("{} - {}", key, output);
            break
        }
        md5.reset()
    }
}
