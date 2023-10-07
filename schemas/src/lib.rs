pub use tonic;

pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

pub mod account {
    include!(concat!(env!("OUT_DIR"), "/account.rs"));
}

pub mod profile {
    include!(concat!(env!("OUT_DIR"), "/profile.rs"));
}
pub mod tweet {
    include!(concat!(env!("OUT_DIR"), "/tweet.rs"));
}
