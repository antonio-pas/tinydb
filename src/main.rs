#![allow(unused)]
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::{
  fs::{File, OpenOptions, self},
  io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
  net::TcpListener,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
async fn save_data(path: &str, data: &[u8]) -> Result<()> {
  let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
  let tmp_name = format!("{}.tmp.{}", path, now);
  let mut file = OpenOptions::new().write(true).create(true).open(&tmp_name).await?;
  file.write(data).await?;
  file.flush();

  fs::rename(tmp_name, path).await?;

  Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8192").await.unwrap();
  while let Ok((stream, _)) = listener.accept().await {
    tokio::task::spawn(async {
      let mut reader = BufReader::new(stream);
      let mut string = String::new();
      reader.read_line(&mut string).await.unwrap();
    })
    .await
    .unwrap();
  }
  Ok(())
}

#[tokio::test]
async fn saves_data() {
  save_data("test.txt", b"hello").await.unwrap();
  let a = fs::read_to_string("test.txt").await.unwrap();
  assert_eq!(a, "hello".to_owned());

  save_data("test.txt", b"world").await.unwrap();
  let b = fs::read_to_string("test.txt").await.unwrap();
  assert_eq!(b, "world".to_owned());
}
