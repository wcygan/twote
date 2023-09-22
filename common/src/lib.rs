use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub enum Service {
    TwoteApi,
    AccountsBackend,
}

impl Service {
    pub fn port(&self) -> u16 {
        match self {
            Service::TwoteApi => 8081,
            Service::AccountsBackend => 8082,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Service::TwoteApi => "twote-api",
            Service::AccountsBackend => "accounts-backend",
        }
    }

    pub fn addr(&self) -> String {
        match self {
            _ => {
                format!("https://{}:{}", self.name(), self.port())
            }
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        SocketAddr::new(ip, self.port())
    }
}
