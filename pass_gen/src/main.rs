use structopt::StructOpt;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


#[derive(Debug, StructOpt)]
#[structopt(name = "sl0w pass-gen", about = "A dumb password generator for learning Rust.")]



struct Opt {
    #[structopt(short = "l", long = "length")]
    length:usize,

    #[structopt(short = "c", long = "complexity")]
    complexity:u8
}

enum Password {
    Low(u8),
    Medium(u8),
    High(u8),
}


fn generate(password: Password){
    let mut rng = rand::thread_rng();

    match password {
        Password::Low(len) => {
            println!("{len} characters with Low complexity");
            let pass: String = (0..len)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET_LOW.len());
                    CHARSET_LOW[idx] as char
                })
            .collect();

            println!("{pass}");

        },
        Password::Medium(len) => {
            println!("{len} characters with Medium complexity");
            let pass: String = (0..len)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET_MED.len());
                    CHARSET_MED[idx] as char
                })
            .collect();
            println!("{pass}");


        }
        ,
        Password::High(len) => {
            println!("{len} characters with High complexity");
            let pass: String = (0..len)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET_HIG.len());
                    CHARSET_HIG[idx] as char
                })
            .collect();
            println!("{pass}");

        }
    }
}



const CHARSET_LOW: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz";

const CHARSET_MED: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789";

const CHARSET_HIG: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789\
                          )(*&^%$#@!~";


fn main() {

    let opt = Opt::from_args();
    let size = opt.length;
    let complexity = opt.complexity;

    match complexity {
        0 => generate(Password::Low(size as u8)),
        1 => generate(Password::Medium(size as u8)),
        2 => generate(Password::High(size as u8)),
        _ => generate(Password::Medium(size as u8)),
    }

}
