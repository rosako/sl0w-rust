use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "sl0w scanner", about = "A dumb port scanner for learning Rust.")]
struct Opt {
    #[structopt(short = "p", long = "port", default_value = "22")]
    port: String,

    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,

    #[structopt(short = "r", long = "range", default_value = "8080,8081", use_delimiter = true)]
    range: Vec<String>
    

}


fn main() {
    let opt = Opt::from_args();
    //println!("{:?}", opt);

    let address = opt.address;
    let port = opt.port;
    let range = opt.range;



    for port in range {
        let target = format!("{address}:{port}");
        if tryPort(target) {
            println!("{} OPEN!", port);
        }else{
            println!("{} CLOSED!", port);
        }
    }

}




fn tryPort(addr_port: String) -> bool {
    //println!("{}", addr_port);
    //let stream = TcpStream::connect(addr_port)
    //    .expect("Can't connect to the host on port");
    //return true;

    match TcpStream::connect(addr_port) {
        Ok(_) => true,
        Err(_) => false,

    }



}



