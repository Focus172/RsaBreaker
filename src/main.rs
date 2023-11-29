// #![allow(unused)]
#![feature(alloc_layout_extra)]
#![feature(iter_map_windows)]
#![feature(array_windows)]

pub mod data;
pub mod error;
pub mod network;
pub mod prelude;
pub mod traits;
pub mod util;
pub mod world;


use crate::prelude::*;

use crate::data::Data;

#[tokio::main]
async fn main() -> Result<()> {
    crate::logger();

    log::debug!("Log init");

    let network = Network::open();

    let mut data = Data::new();

    while let Some((publ_key, priv_key)) = data.next().await {
        let inpt = publ_key.as_training_data();
        let targ = priv_key.as_training_data();

        // dbg!(&targ);

        network.train(inpt.into_vec(), targ.into_vec(), 0.1);

        // dbg!(network.output());
    }

    let f = fs::File::create("nn.yaml").unwrap();
    yaml::to_writer(f, &network).unwrap();
    // json::to_writer_pretty(f, &network)?;

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
