use message::{serialize_with_capacity, deserialize_with_capacity, BytesMut, SimpleText};
use std::{
    io::stdin,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;

pub type Exception = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Exception>;

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("127.0.0.1:50000").await?;
    let (_, mut writer) = io::split(stream);
    let mut buffer = BytesMut::with_capacity(8196);
    let mut data = SimpleText {
        channel: "".into(),
        contents: "".into(),
    };

    // tokio::spawn(async move {
    //     loop {
    //         let n = match reader.read(&mut buffer).await {
    //             Ok(n) => n,
    //             Err(e) => {
    //                 info!("{:?}", e);
    //                 break;
    //             }
    //         };
    //         if n == 0 {
    //             break;
    //         }
    //         deserialize_with_capacity(&mut buffer, &mut *data.lock().unwrap()).unwrap();
    //     }
    // });
    
    // User input.
    
    loop {
        let mut channel = String::new();
        stdin().read_line(&mut channel).unwrap();
        println!("{}", &channel);

        let mut contents = String::new();
        stdin().read_line(&mut contents).unwrap();
        println!("{}", &contents);

        data.channel = channel;
        data.contents = contents;

        serialize_with_capacity(&mut buffer, &mut data)?;
        println!("{:?}", buffer);
        writer.write_buf(&mut buffer).await?;
    }
}
