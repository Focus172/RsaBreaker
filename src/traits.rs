use std::{mem, slice};

use rsa::{RsaPrivateKey, RsaPublicKey};

static U8_MAX: f32 = u8::MAX as f32;

pub trait AsTrainingData: Sized {
    unsafe fn as_bytes(&self) -> &[u8] {
        slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>())
    }

    fn as_training_data(&self) -> Box<[f32]> {
        unsafe { self.as_bytes() }
            .into_iter()
            .map(|b| *b as f32 / U8_MAX)
            .collect()
    }
}

impl AsTrainingData for RsaPublicKey {}
impl AsTrainingData for RsaPrivateKey {}
