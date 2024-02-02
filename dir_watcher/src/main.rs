use std::fs;
extern crate walkdir;
use walkdir::WalkDir;


fn main() {
    println!("Crawl :");
    crawl();

    println!("Crawl2 :");
    //crawl2();
}


fn crawl(){
    for file in fs::read_dir("/home/urk3l/").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}


fn crawl2(){
    for file in WalkDir::new("./").into_iter().filter_map(|file| file.ok()) {
        println!("{}", file.path().display());
    }
}
