use serde_json::{Map, Value};
use std::{collections::HashMap, fs, net::UdpSocket};

use monitor_server::common::run;

fn get_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

fn parse_config(map: &Map<String, Value>) -> HashMap<String, String> {
    let mut re: HashMap<String, String> = HashMap::new();
    for (key, value) in map {
        re.insert(String::from(key), String::from(value.as_str().unwrap()));
    }
    return re;
}

#[actix_web::main]
async fn main() {
    log4rs::init_file("./configs/log4rs.yaml", Default::default()).unwrap();

    let ip = get_ip().unwrap();

    let config_db: Value =
        serde_json::from_str(&fs::read_to_string("./configs/database.json").unwrap()).unwrap();

    let config_db = parse_config(config_db.as_object().unwrap());

    run::server(ip, config_db).await.unwrap();
}
