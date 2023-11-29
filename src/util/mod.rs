pub mod rand;
pub mod aslice;

// fn sized_seq<const N: usize>() -> [usize; N] {
//     // SAFETY: MaybeUninit does not have to be initialized
//     let mut ret: [MaybeUninit<usize>; N] = unsafe { MaybeUninit::uninit().assume_init() };
//
//     ret.iter_mut().enumerate().for_each(|(i, e)| {
//         e.write(i);
//     });
//
//     unsafe { ret.transpose().assume_init() }
// }
