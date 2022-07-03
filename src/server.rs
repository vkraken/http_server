pub struct Server {
    addr: String,
}

impl Server {
    // associated method (like static method)
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    
    // real method like class method
    pub fn run(self) {
        println!("Listening on {}", self.addr);
    }
}
