use structopt::StructOpt;
use std::{fs, fs::File,  path::Path};
use sha256::{digest, try_digest};
use std::io::{Read, BufReader};
use md5::{Md5, Digest};
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256};
use blake2::{Blake2b512, Blake2s256};
use sha1::{Sha1};
use hex_literal::hex;


#[derive(Debug, StructOpt)]
#[structopt(name = "sl0w hash-calc", about = "A dumb hash calculator for learning Rust.")]

struct Opt {
    #[structopt(short = "f", long = "file")]
    file_path: String,

    #[structopt(short = "a", long = "algo", default_value = "md5")]
    algo: String
}



// just for testing purpose
fn readfiletest(){

    let buffer = fs::read("Cargo.toml");

    let mut hasher = Md5::new();
    hasher.update(b"Hello world");
    let result = hasher.finalize();

    println!("{:x}",result);

    let mut hasher2 = Sha256::new();
    hasher2.update(b"Hello world");
    let result2 = hasher2.finalize();
    println!("{:x}", result2);

    let mut hasher3 = Sha1::new();
    hasher3.update(b"Hello world");
    let result3 = hasher3.finalize();
    println!("{:x}", result3);

    let mut hasher4 = Blake2b512::new();
    hasher4.update(b"Hello world");
    let result4 = hasher4.finalize();
    println!("{:x}", result4);


    let file = "Cargo.toml";
    let input_raw: String = fs::read_to_string(file).unwrap();
    
}



fn fileToBuf(file_path: &str) -> std::io::Result<Vec<u8>>{
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() {
    let opt = Opt::from_args();

    let file_path = opt.file_path;
    //let input = Path::new(&file_path);

    let mut bytes: Vec<u8> = Vec::new();

    match fileToBuf(file_path.as_str()){
        Ok(content) => {
            bytes = content;
        }
        Err(e) => eprintln!("Error reading the file: {}", e),

    };


    let algo = opt.algo;

    match algo.as_str() {
        "md5" =>{
            let mut hasher = Md5::new();
            hasher.update(bytes);
            let result = hasher.finalize();
            println!("MD5 <{:x}>", result);
        },
        "sha1" =>{
            let mut hasher = Sha1::new();
            hasher.update(bytes);
            let result = hasher.finalize();
            println!("SHA1 <{:x}>", result);


        },
        "sha2" =>{
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            let result = hasher.finalize();
            println!("SHA256 <{:x}>", result);


        },
        "blake2" =>{
            let mut hasher = Blake2b512::new();
            hasher.update(bytes);
            let result = hasher.finalize();
            println!("BLAKE2 <{:x}>", result);


        }
        _ => {
            //should never get here, as I set a default value to algo
            let mut hasher = Md5::new();
            hasher.update(bytes);
            let result = hasher.finalize();
            println!("No algorithm provided defaulting to MD5!\nMD5 <{:x}>", result);

        }




    }


}
