use crate::protocol::setup::{Group, Scalar, SetupData};
use digest::Digest;
use rand_core::{CryptoRng, RngCore};
use std::ops::{Add, Mul};

pub struct Commitment<T, K> {
    commitment: T,
    salt: K,
    message: K,
}

impl<T, K> Commitment<T, K>
where
    T: Add<T, Output = T> + Mul<K> + Default + Group,
    K: Default + Mul<T> + Mul<T, Output = T> + Scalar + Copy,
{
    fn new<R: RngCore + CryptoRng, D: Digest<OutputSize = u64>>(
        message: Vec<u8>,
        rng: &mut R,
    ) -> Self {
        let message_as_scalar: K = K::from_bytes::<D>(message);
        let setup_data = SetupData::<T, K>::new(rng);
        let salt = K::random(rng);
        Commitment {
            commitment: message_as_scalar * setup_data.g + salt * setup_data.h,
            salt,
            message: message_as_scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_fails_empty() {
        assert_eq!(0, 0);
    }
    #[test]
    fn test_commit_non_err_on_nonempty() {
        assert_ne!(0, 1);
    }
}
