#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting e3-mini");   
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await?;

    loop {
        let (mut steam, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0;1024];

            match steam.read() {
                
            }
    }
        

    Ok(())
}
