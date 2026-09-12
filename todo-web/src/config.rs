use std::net::{IpAddr, Ipv4Addr};

use todo_macro::build_config;

build_config! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Config {
        pub api_url: String = format!("http://{}:{}", IpAddr::V4(Ipv4Addr::LOCALHOST), 7630),
    }
}
