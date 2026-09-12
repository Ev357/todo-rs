use std::net::{IpAddr, Ipv4Addr};

use crate::build_config;

mod build_config;

build_config! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Config {
        pub address: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        pub port: u16 = 3000,
        pub database_url: String,
    }
}
