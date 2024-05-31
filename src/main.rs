use crate::base::{
    health_server::HealthServer, hello_server::HelloServer, numbers_server::NumbersServer,
    translations_server::TranslationsServer,
};
use log::info;
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode};
use std::{fs::File, io::Read};
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

pub mod health {
    tonic::include_proto!("health");
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
    info!("Server starting at {addr}");

    // Read the generated descriptor set
    let mut file = File::open("proto/base_descriptor.bin")?;
    let mut buf = Vec::new();
    file.read_to_end(buf.as_mut())?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(&buf)
        .build()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(TranslationsServer::new(
            handler::translations::TranslationsServer::new(db),
        ))
        .add_service(HealthServer::new(handler::health::HealthServer::default()))
        .add_service(HelloServer::new(handler::hello::HelloServer::default()))
        .add_service(NumbersServer::new(
            handler::numbers::NumbersServer::default(),
        ))
        .serve(addr)
        .await?;

    Ok(())
}
