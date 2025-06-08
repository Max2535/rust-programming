use tokio::{net::TcpListener, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}};
use tokio::sync::broadcast;

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    let (tx, _) = broadcast::channel::<String>(100);

    println!("🚀 Chat Server running at 127.0.0.1:9000");

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut lines = BufReader::new(reader).lines();

            // Task: รับข้อความจาก broadcast แล้วส่งกลับ client
            let mut write_task = tokio::spawn(async move {
                while let Ok(msg) = rx.recv().await {
                    let _ = writer.write_all(msg.as_bytes()).await;
                }
            });

            // รับ input จาก client นี้
            while let Ok(Some(line)) = lines.next_line().await {
                let msg = format!("{}: {}\n", addr, line);
                let _ = tx.send(msg); // broadcast ให้ทุกคน
            }

            write_task.abort();
        });
    }
}
