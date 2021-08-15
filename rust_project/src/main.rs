use std::time::Instant;

const HELLO_WORLD:&str = "hello_world";
const SUM:&str = "sum";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let action: &str = match args.get(1){
        None => HELLO_WORLD,
        Some(s) => s.as_str(),
    };
    exec(action);
}

fn exec(action: &str) {
    let func = match action {
        HELLO_WORLD => hello_world,
        SUM => sum,
        _ => { println!("action '{}' not found", action);||{}},
    };
    exec_with_benchmark(action, func);
}

fn exec_with_benchmark<F>(action_name: &str, f:F) where F: Fn() {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    println!("action {} elapsed {:#?} microseconds", action_name, elapsed.as_micros());
}

fn hello_world() {
    println!("Hello, world!")
}

fn sum() {
    let a = 2 + 3;
    assert_eq!(a, 5)
}