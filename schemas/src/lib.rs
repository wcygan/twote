pub use tonic;

pub mod account {
    include!(concat!(env!("OUT_DIR"), "/account.rs"));
}

pub mod profile {
    include!(concat!(env!("OUT_DIR"), "/profile.rs"));
}

pub mod tweet {
    include!(concat!(env!("OUT_DIR"), "/tweet.rs"));
}
