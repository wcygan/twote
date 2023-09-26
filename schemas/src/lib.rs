pub use tonic;

pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

pub mod account {
    include!(concat!(env!("OUT_DIR"), "/account.rs"));
}
