use std::env;
use txl_rs::txl;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some((_head, tail)) = args.split_first() {
        match txl(tail.to_vec()) {
            Ok(result) => {
                println!("{result}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
}
