use rand::Rng;

pub fn weights(number: usize, min: f32, max: f32) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..number).map(|_| rng.gen_range(min..=max)).collect()
}
