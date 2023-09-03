mod node;
mod layer;
mod network;

pub use self::network::Network;
pub use self::layer::Layer;
pub use self::node::Node;

pub struct KeyRange {
    factor_1_lowerbound: i64,
    factor_1_upperbound: i64,
    factor_2_lowerbound: i64,
    factor_2_upperbound: i64,
}


