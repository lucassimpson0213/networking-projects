fn main() {
    let ip = std::env::args().nth(1).expect("no ip given");
    let mask = std::env::args().nth(2).expect("no mask given");
    let argslen = std::env::args().len();
    if argslen != 3 {
        println!("usage: ip <ip> <mask>");   
    }

    
    

    
}



// so the first thing that we are going to do is take in an ip address and out will come what info
// we can derive from it
