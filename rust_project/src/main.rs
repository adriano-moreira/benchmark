fn main() {
    let args: Vec<String> = std::env::args().collect();
    hello_world();
}


fn hello_world() {
    println!("Hello, world!");
}
