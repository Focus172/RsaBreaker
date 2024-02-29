use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// ---
#[derive(Serialize, Deserialize)]
pub struct Uuid(usize);

impl Distribution<Uuid> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Uuid {
        Uuid(rng.gen())
    }
}
