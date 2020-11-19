use tokio::time;
use tokio::process::Command;
use std::process::{ Stdio };

async fn task_that_takes_a_second(kind: Command) {
    kind.stdin(Stdio::piped().lock().write_all(b"Test"));
    time::sleep(time::Duration::from_secs(1)).await
}

#[tokio::main]
async fn main() {
    // let mut stdout = Stdio::piped();
    // let mut stdin = Stdio::piped();
    let mut process = Command::new("child/target/debug/child.exe");

    let mut interval = time::interval(time::Duration::from_secs(2));
    for _i in 0..5 {
        interval.tick().await;
        task_that_takes_a_second(process).await;
    }
}

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


