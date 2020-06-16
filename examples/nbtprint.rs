extern crate nbt;
extern crate serde_json;

use std::env;
use std::fs;
use std::process::exit;

use nbt::Blob;
use nbt::Result;

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.into_iter().skip(1).take(1).next() {
        let mut file = fs::File::open(&arg)?;
        println!(
            "================================= NBT Contents ================================="
        );
        let blob = Blob::from_reader(&mut file)?;
        println!("{}", blob);
        println!(
            "============================== JSON Representation ============================="
        );
        match serde_json::to_string_pretty(&blob) {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("error: {}", e);
                exit(1)
            }
        }
        Ok(())
    } else {
        eprintln!("error: a filename is required.");
        exit(1)
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        exit(1)
    };
}
