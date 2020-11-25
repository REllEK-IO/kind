// Transfer lib.rs from child_test here
// Use an async channel to link to child process

mod kinder {
    // use tokio::prelude::*;
    // use tokio::time;
    // use tokio::process::{Child, Command};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncWrite, AsyncRead, BufReader};
    use std::error::Error;
    use std::fmt;
    // use std::io::{ self, Write, Read };
    // use std::process::{ChildStdin, ChildStdout, Stdio};
    // use futures::*;
    // use std::sync::mpsc::channel;
    // use std::thread;
    pub struct Kind {
        pub path: String,
    }

    impl Default for Kind {
        fn default() -> Self { Self { path: String::from("Hello World") } }
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

    pub fn kind () -> Result<Kind, String> {
        Ok(Kind::default())
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
        let kind = kind().unwrap();

        assert_eq!(kind.path, "Hello World");
    }
}