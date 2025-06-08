use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpStream};

pub async fn run_client() {
    let stream = TcpStream::connect("127.0.0.1:9000").await.unwrap();
    let (reader, mut writer) = stream.into_split();
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    let mut socket_reader = BufReader::new(reader).lines();

    println!("✅ Connected to chat");

    // รับจาก server
    tokio::spawn(async move {
        while let Ok(Some(line)) = socket_reader.next_line().await {
            println!(">> {}", line);
        }
    });

    // ส่ง input ของ user ไปให้ server
    while let Ok(Some(line)) = stdin.next_line().await {
        writer.write_all(format!("{}\n", line).as_bytes()).await.unwrap();
    }
}
