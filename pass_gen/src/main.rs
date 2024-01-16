use structopt::StructOpt;
use rand::prelude::*;


#[derive(Debug, StructOpt)]
#[structopt(name = "sl0w pass-gen", about = "A dumb password generator for learning Rust.")]



struct Opt {
    #[structopt(short = "l", long = "length")]
    length:u8,

    #[structopt(short = "c", long = "complexity")]
    complexity:u8
}



fn main() {
    println!("Hello, world!");


    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);


}
