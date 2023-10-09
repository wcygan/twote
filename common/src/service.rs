use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub enum Service {
    TwoteApi,
    AccountsBackend,
    ProfilesBackend,
    TweetsBackend,
}

impl Service {
    pub fn port(&self) -> u16 {
        match self {
            Service::TwoteApi => 8081,
            Service::AccountsBackend => 8082,
            Service::ProfilesBackend => 8083,
            Service::TweetsBackend => 8084,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Service::TwoteApi => "twote-api",
            Service::AccountsBackend => "accounts-backend",
            Service::ProfilesBackend => "profiles-backend",
            Service::TweetsBackend => "tweets-backend",
        }
    }

    pub fn addr(&self) -> String {
        format!("https://{}:{}", self.name(), self.port())
    }

    pub fn socket_addr(&self) -> SocketAddr {
        let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        SocketAddr::new(ip, self.port())
    }
}
