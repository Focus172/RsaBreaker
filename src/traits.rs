use rsa::{RsaPrivateKey, RsaPublicKey};

static U8_MAX: f32 = u8::MAX as f32;

pub trait AsTrainingData: Sized {
    /// Coverts a type into a slice of bytes
    ///
    /// # Safety
    /// This function is safe on most types. I think.
    // unsafe fn as_bytes(&self) -> &[u8] {
    //     slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>())
    // }
    fn as_bytes(&self) -> Box<[u8]>;

    fn as_training_data(&self) -> Box<[f32]> {
        self.as_bytes()
            .into_vec()
            .into_iter()
            .map(|b| b as f32 / U8_MAX)
            .collect()
    }
}

impl AsTrainingData for RsaPublicKey {
    fn as_bytes(&self) -> Box<[u8]> {
        use rsa::traits::PublicKeyParts;
        self.n()
            .to_bytes_le()
            .into_iter()
            .chain(self.e().to_bytes_le())
            .collect()
    }
}

impl AsTrainingData for RsaPrivateKey {
    fn as_bytes(&self) -> Box<[u8]> {
        use rsa::traits::PrivateKeyParts;
        self.primes().iter().flat_map(|p| p.to_bytes_le())
            .chain(self.d().to_bytes_le())
            .collect()
    }
}
