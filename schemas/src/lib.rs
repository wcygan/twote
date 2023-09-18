pub use tonic;

pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

pub mod login {
    include!(concat!(env!("OUT_DIR"), "/login.rs"));
}
