extern crate exonum;

mod blockchain;
mod api;
mod service;


fn main() {
    exonum::crypto::init();
    exonum::helpers::init_logger().unwrap();

    println!("Hello, world!");
}
