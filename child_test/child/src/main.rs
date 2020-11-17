use std::process;
// use std::io:Stdin;

// fn main() {
    
//     println!("Hello, world! {}", process::id());
    
// }


use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer);

    let stdout = io::stdout();
    let mut handle_out = stdout.lock();

    let message = format!("Fuck yeah! {}", buffer);
    handle_out.write_all(message.as_bytes());
    // println!("Hello, world! {}", buffer);
}