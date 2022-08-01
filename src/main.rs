use std::env;

pub mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let mut machine = interpreter::Interpreter::default();

        for arg in &args[1..] {
            machine.process_instructions(arg.as_str());
        }
        
    } else {
        println!("No file provided.");
    }
}
