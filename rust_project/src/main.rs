use std::time::Instant;

mod nothing;
mod multiply_table;

const HELLO_WORLD: &str = "hello_world";
const NOTHING: &str = "nothing";
const MULTIPLY_TABLE: &str = "multiply_table";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let action: &str = match args.get(1) {
        None => HELLO_WORLD,
        Some(s) => s.as_str(),
    };
    exec(action);
}

fn exec(action: &str) {
    let func:Option<fn()->()> = match action {
        HELLO_WORLD => Some(hello_world),
        MULTIPLY_TABLE => Some(multiply_table::exec),
        NOTHING => Some(nothing::exec),
        _ => None,
    };

    match func {
        Some(f) => exec_with_benchmark(action, f),
        None => println!("action '{}' not found", action),
    }

}

fn exec_with_benchmark<F>(action_name: &str, f: F) where F: Fn() {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    println!("action {} elapsed {:#?} microseconds", action_name, elapsed.as_micros());
}

fn hello_world() {
    println!("Hello, world!")
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test() {
        exec(MULTIPLY_TABLE);
    }
}