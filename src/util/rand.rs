use rand::Rng;

pub fn weights(size: usize, min: f32, max: f32) -> Box<[f32]> {
    let mut rng = rand::thread_rng();
    std::iter::repeat_with(|| rng.gen_range(min..=max))
        .take(size)
        .collect()
}
