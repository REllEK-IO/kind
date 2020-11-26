// Transfer lib.rs from child_test here
// Use an async channel to link to child process

mod kinder {
    // use tokio::prelude::*;
    // use tokio::time;
    use tokio::process::{Child, Command};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncWrite, AsyncRead, BufReader};
    use std::error::Error;
    use std::fmt;
    use async_channel::{ self, Sender, Receiver};
    // use std::io::{ self, Write, Read };
    use std::process::{ChildStdin, ChildStdout, Stdio};
    // use futures::*;
    // use std::sync::mpsc::channel;
    // use std::thread;
    pub struct Kind {
        pub path: String,
        cmd: Child,
        sender: Sender<String>,
        receiver: Receiver<String>,
    }

    impl Default for Kind {
        fn default() -> Self {
            let (sender, receiver) = async_channel::unbounded();
            Self { 
                path: String::from("Hello World"),
                cmd: Command::new("").spawn().unwrap(),
                sender: sender,
                receiver: receiver
            } 
        }
    }

    impl fmt::Debug for Kind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Path: {}", self.path)
        }
    }

    impl Kind {
        async fn dispatch(&self) {}
        async fn subscribe(&self) {}
    }

    pub fn kind (path: &str) -> Result<Kind, String> {
        let (sender, receiver) = async_channel::unbounded();
        let mut child_process = Command::new("child/target/debug/child.exe");
        child_process
            .stdout(Stdio::piped())
            .stdin(Stdio::piped());
        let child: Child;
        match child_process.spawn() {
            Ok(c) => child = c,
            _ => return Err(String::from("Error starting child from path")),
        };
        Ok(Kind { 
            path: String::from(path),
            cmd: child,
            sender: sender,
            receiver: receiver,
        })
    }

    pub async fn kind_write<W: AsyncWrite + Unpin, U: Unpin>(
        mut kind_in: W,
        str: String
    ){
        kind_in.write_all(str.as_bytes()).await.unwrap();
    }

    // pub async fn kind_read<R: AsyncRead>(
    //     kind_out: R,

    // ) -> String {
    //     let mut reader = BufReader::new(kind_out).lines();

    //     let data = reader.next_line().await;
    //     String::from("TEST")
    // }
}

#[cfg(test)]
mod tests {
    use super::kinder::*;
    #[test]
    fn creates() {
        let kind = kind("").unwrap();

        assert_eq!(kind.path, "");
    }
}