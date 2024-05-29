use crate::base::root_server::RootServer;
use log::info;
use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
use tonic::transport::Server;

pub mod hello {
    tonic::include_proto!("hello");
}
pub mod base {
    tonic::include_proto!("base");
}
pub mod add_numbers {
    tonic::include_proto!("add_numbers");
}
pub mod language {
    tonic::include_proto!("language");
}

mod db;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term_config = simplelog::ConfigBuilder::new()
        .set_location_level(LevelFilter::Debug)
        .add_filter_ignore_str("h2")
        .build();

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        term_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let db = db::DB::new().await?;
    let addr = "0.0.0.0:8080".parse()?;
    info!("Server started at {addr}");

    let server = handler::my_server::MyServer::new(db);
    Server::builder()
        .add_service(RootServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
