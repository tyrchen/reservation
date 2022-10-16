use abi::Config;
use anyhow::Result;
use reservation_service::start_server;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let filename = std::env::var("RESERVATION_CONFIG").unwrap_or_else(|_| {
        let p1 = Path::new("./reservation.yml");
        let path = shellexpand::tilde("~/.config/reservation.yml");
        let p2 = Path::new(path.as_ref());
        let p3 = Path::new("/etc/reservation.yml");

        match (p1.exists(), p2.exists(), p3.exists()) {
            (true, _, _) => p1.to_str().unwrap().to_string(),
            (_, true, _) => p2.to_str().unwrap().to_string(),
            (_, _, true) => p3.to_str().unwrap().to_string(),
            _ => panic!("no config file found"),
        }
    });

    let config = Config::load(filename)?;
    start_server(&config).await
}
