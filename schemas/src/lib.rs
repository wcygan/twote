pub use tonic;

pub mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}
