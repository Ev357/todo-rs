use std::net::{IpAddr, Ipv4Addr};

use todo_macro::build_config;

build_config! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Config {
        pub address: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST),
        pub port: u16 = 7630,
        pub database_url: String,
    }
}
