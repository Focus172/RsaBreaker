use core::{mem, slice};

use rsa::{RsaPublicKey, RsaPrivateKey};

static U8_MAX: f32 = u8::MAX as f32;

pub trait AsTrainingData: Sized {
    fn as_bytes(&self) -> Box<[u8]> {
        unsafe { slice::from_raw_parts((self as *const Self) as *const u8, mem::size_of::<Self>()) }
            .iter()
            .cloned()
            .collect()
    }

    fn as_training_data(&self) -> Box<[f32]> {
        self.as_bytes().iter().map(|b| *b as f32 / U8_MAX).collect()
    }
}

impl AsTrainingData for RsaPublicKey {}
impl AsTrainingData for RsaPrivateKey {}
