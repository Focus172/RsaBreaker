// #![allow(unused)]
// #![feature(alloc_layout_extra)]
// #![feature(iter_map_windows)]
#![feature(array_windows)]
#![feature(lazy_cell)]
#![feature(yeet_expr)]

pub mod data;
pub mod error;
pub mod network;
pub mod prelude;
pub mod traits;
pub mod util;
pub mod world;

use std::io;

use crate::network::Network;
// use crate::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    logger();
    log::debug!("Log init");

    let mut network = Network::new("nn.json");

    network.train().await;

    network.save("nn.json")?;

    Ok(())
}

fn logger() {
    simplelog::TermLogger::init(
        log::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();
}

// let mut iter = output.into_iter();
// let n = iter.next().unwrap();
// let e = iter.next().unwrap();
// let d = iter.next().unwrap();
// let primes = iter.filter(|i| *i < BigUint::new(vec![3])).collect();
// rsa::RsaPrivateKey::from_components(n, e, d, primes).unwrap()
