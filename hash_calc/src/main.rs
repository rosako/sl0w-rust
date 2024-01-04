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

    #[structopt(short = "a", long = "algo", default_value = "sha2")]
    algo: String
}


fn readfiletest(){

    let buffer = fs::read("Cargo.toml");

    let mut hasher = Md5::new();
    hasher.update(b"Hello world");
    let result = hasher.finalize();

    println!("{:x}",result);

    let mut hasher2 = Sha256::new();
    let mut hasher3 = Sha1::new();
    let mut hasher4 = Blake2b512::new();

    let file = "Cargo.toml";
    let input_raw: String = fs::read_to_string(file).unwrap();
    
    
    //let val = digest(buffer);

    //println!("readfiletest result => {val}");

}


fn fileToBuf(file_path: &str) -> std::io::Result<Vec<u8>>{
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);
    return Ok(buf);
}

/*
fn get_hash(file_path: &str, algo: &str) -> String{
    let input = Path::new(&file_path);
    let mut hasher;
    match algo {
        "sha2" => {
            hasher = Sha256::new();
        },
        "md5" => {
            //hasher = Md5::new();
            hasher = Sha256::new();
        },
        "sha1" => {
            hasher = Sha1::new();

        },
        "sha3" => {
            hasher = Sha3_256::new();
        },
        "blake2" =>{
            hasher = Blake2b512::new();
            
        }
        _ => hasher = Md5::new();
    }

    "Test String".to_string()
}
*/

fn main() {
    let opt = Opt::from_args();

    let file_path = opt.file_path;
    //let input = Path::new(&file_path);

    let algo = opt.algo;

    let buf = fileToBuf(file_path.as_str());
    let mut hasher = Md5::new();
    hasher.update(buf.bytes());
    //let result = hasher.finalize();
    //println!("{:x}", result);

    //let test = get_hash(&file_path, &algo);

    //println!("algo = {algo}\n\t\t\t hash => {test}");

    readfiletest();
}
