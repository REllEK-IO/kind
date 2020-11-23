use tokio::prelude::*;
// use tokio::time;
use tokio::process::{Child, Command};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncWrite, BufReader};
// use std::io::{ self, Write, Read };
use std::process::{ChildStdin, Stdio};
use futures::*;

use kind::kinder;

#[tokio::main]
async fn main() {
    let mut child_process = Command::new("child/target/debug/child.exe");

    child_process
        .stdout(Stdio::piped())
        .stdin(Stdio::piped());

    let mut child = child_process.spawn().unwrap();

    let kind_stdout = child.stdout.take().unwrap();
    let mut kind_stdin = child.stdin.take().unwrap();

    let write = async {
        kind_stdin.write_all(b"Hello Kind!").await.unwrap();
        kinder::kind_write(kind_stdin, String::from("Hello) World")).await;
        
        // drop(kind_stdin);
    };

    let read = async {
        let mut reader = BufReader::new(kind_stdout).lines();
        let mut num_lines = 0;
        let n = 10_usize;
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
            println!("HI!");
        }
    };
    

    // drop(child.stdin.take());
    // drop(child.stdout.take());
    // drop(child.stderr.take());

    future::join(write, read).await;
        // .map(|(_, _, status)| status)
}

// async fn task_that_takes_a_second(i: usize, kind: &Command) {
//     let stdout = io::stdout();
//     let mut handle = stdout.lock();
//     stdout.write(b"TEST");
//     let stdout = kind.stdout(stdout);
//     // handle.write_all(format!("{}\n", i).as_bytes());
//     time::sleep(time::Duration::from_secs(1)).await
// }

// #[tokio::main]
// async fn main() {
//     // let mut stdout = Stdio::piped();
//     // let mut stdin = Stdio::piped();
//     let mut process = Command::new("child/target/debug/child.exe");

//     let mut interval = time::interval(time::Duration::from_secs(2));
//     for i in 0..5 {
//         interval.tick().await;
//         task_that_takes_a_second(i, &process).await;
//     }
// }

// use std::io::prelude::*;
// use std::process::{Command, Stdio};

// static PANGRAM: &'static str =
// "the quick brown fox jumped over the lazy dog\n";

// fn main() {
//     let mut process = match Command::new("child/target/debug/child.exe")
//         .stdin(Stdio::piped())
//         .stdout(Stdio::piped())
//         .spawn() {
//         Err(why) => panic!("couldn't spawn wc: {}", why),
//         Ok(process) => process,
//     };

//     match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
//         Err(why) => panic!("couldn't write to wc stdin: {}", why),
//         Ok(_) => println!("sent pangram to wc"),
//     }

//     // process.stdin.unwrap().write_all(TESTER.as_bytes());

//     let mut s = String::new();
//     match process.stdout.unwrap().read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read wc stdout: {}", why),
//         Ok(_) => print!("wc responded with:\n{}", s),
//     }
// }


