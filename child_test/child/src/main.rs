use tokio::prelude::*;
use tokio::io;
// use std::io::{ BufReader, BufRead };

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    stdin.read_to_string(&mut buffer).await?;

    stdout.write_all(&mut buffer.as_bytes()).await?;
    Ok(())
}

// fn main() {
//     let mut buffer = String::new();
//     let stdin = io::stdin();
//     let mut handle = stdin.lock();

//     handle.read_to_string(&mut buffer);

//     let stdout = io::stdout();
//     let mut handle_out = stdout.lock();

//     let message = format!("Fuck yeah! {}", buffer);
//     handle_out.write_all(message.as_bytes());
//     // println!("Hello, world! {}", buffer);
// }