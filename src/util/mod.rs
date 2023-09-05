pub mod rand;

fn sized_seq<const N: usize>(indices: [usize; N]) -> [usize; N] {
    let mut a = [0; N];

    for (i, thing) in a.iter_mut().enumerate() {
        *thing = i;
    }

    a
}
