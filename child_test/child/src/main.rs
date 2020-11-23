use tokio::prelude::*;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use futures::*;
// use std::io::{ BufReader, BufRead };

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let write = async {
        loop {
            if buffer.len() == 0 {
                continue;
            } else {
                stdout.write_all(buffer.as_bytes()).await.unwrap();
            }
        };
        
        drop(stdout);
    };

    let read = async {
        let mut reader = BufReader::new(stdin);
        
        // Try to read `n + 1` lines, ensuring the last one is empty
        // (i.e. EOF is reached after `n` lines.
        loop {
            let data = reader
                .next_line()
                .await
                .unwrap_or_else(|_| Some(String::new()))
                .expect("failed to read line");

            let num_read = data.len();
            let done = num_lines >= n;

            match (done, num_read) {
                (false, 0) => panic!("broken pipe"),
                (true, n) if n != 0 => panic!("extraneous data"),
                _ => {
                    let expected = format!("line {}", num_lines);
                    assert_eq!(expected, data);
                }
            };
        }
    };
    future::join(write, read).await;
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