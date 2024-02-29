pub mod driver;
mod layer;
mod node;

use crate::prelude::*;

pub use self::layer::Layer;
pub use self::node::Node;

use crate::data::Data;
use crate::util::iter::Stream;

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    driver: Driver,
    world: World,
    data: Data,
}

impl Network {
    /// Makes a new network from a file or a random one if that fails
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self::try_new(path).unwrap_or_else(|e| {
            log::error!("Network fetch failure: {}", e);
            log::info!("Creating bare network to remedy");

            let mut world = World::new();
            let data = Data::new(400);
            let driver = Driver::random(&mut world, [7, 5, 5, 3]);
            Self {
                driver,
                world,
                data,
            }
        })
    }

    /// Attempts to create a new Network from a path.
    pub fn try_new(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let f = fs::File::open(path)?;
        let n = json::from_reader::<_, Self>(f)?;

        Ok(n)
    }

    pub async fn train(&mut self) {
        while let Some((publ_key, priv_key)) = self.data.next().await {
            let inpt = publ_key.as_training_data().into_vec();
            let targ = priv_key.as_training_data().into_vec();

            self.driver.train(&mut self.world, inpt, targ, 0.1);

            // dbg!(self.driver.output(self.world.proxy()));
        }
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>) -> io::Result<()> {
        json::to_writer_pretty(fs::File::create(path)?, self)?;
        Ok(())
    }
}
