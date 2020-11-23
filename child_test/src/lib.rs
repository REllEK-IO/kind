pub mod kinder {
    use tokio::prelude::*;
    // use tokio::time;
    use tokio::process::{Child, Command};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncWrite, AsyncRead, BufReader};
    // use std::io::{ self, Write, Read };
    use std::process::{ChildStdin, ChildStdout, Stdio};
    use futures::*;
    use std::sync::mpsc::channel;
    use std::thread;

    pub async fn kind_write<W: AsyncWrite + Unpin, U: Unpin>(
        mut kind_in: W,
        str: String
    ){
        kind_in.write_all(str.as_bytes()).await.unwrap();
    }

    pub async fn kind_read<R: AsyncRead>(
        kind_out: R,

    ) -> String {
        let mut reader = BufReader::new(kind_out).lines();

        let data = reader.next_line().await;
        String::from("TEST")
    }
}