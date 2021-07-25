use tokio::{io::AsyncBufReadExt, io::AsyncWriteExt, io::BufReader, net::TcpListener};

const LOCAL_SERVER: &str = "127.0.0.1:8888";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(LOCAL_SERVER).await?;
    loop{
        let (mut socket, addr) = listener.accept().await?;
        println!("{} is connected", addr); 
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut msg = String::new();
            loop{
                let bytes_read = reader.read_line(&mut msg).await.unwrap();
                if bytes_read == 0{
                    break;
                }
                println!("{}", msg);
                writer.write_all(msg.as_bytes()).await.unwrap();
                msg.clear();
            }
        });
    }
}
