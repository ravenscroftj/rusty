use toml;
use rustc_serialize::{Encodable,Decodable};
use std::io::{IoError,File};
use std::str;
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;

#[deriving(RustcEncodable, RustcDecodable)]
pub struct RustyConfig {
    servers: HashMap<String, ServerConfig>
}

#[deriving(RustcEncodable, RustcDecodable)]
pub struct ServerConfig {
    address: String,
    port: int,
    nick: String,
    channels: Vec<String>,
    admins: Vec<String>
}

enum RustyConfigError{
    IoError(IoError),
    ParserErrors(Vec<toml::ParserError>)
}

pub fn load_config( config_path: Path )  -> Result<RustyConfig, RustyConfigError> {
    
    //read the file
    let buff = File::open(&config_path).read_to_string().unwrap();
    let mut parser = toml::Parser::new(buff.as_slice());

    let tomlval = match parser.parse() {
        Some(v) => v,
        None => {
            println!("Parse errors in config {}", parser.errors);
            return Err(RustyConfigError::ParserErrors(parser.errors))
        }
    };

    let mut d = toml::Decoder::new(toml::Value::Table(tomlval));

    match Decodable::decode(&mut d) {
        Ok(val) => Ok(val),
        Err(err) => {
            println!("Error parsing config {}", err.to_string());
            return Err(err)
        }
    };
}

