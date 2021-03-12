use crate::errors::error::CommitError;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

use rand_core::{CryptoRng, RngCore};

pub trait PedersenScalar {
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self;
}
pub trait PedersenGroup {
    fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self;
    fn generator() -> Self;
}

pub struct PedersenGens<T, K>
where
    T: Add<T> + Default,
{
    pub g: T,
    pub h: T,
    _marker: PhantomData<K>,
}

impl<T, K> Default for PedersenGens<T, K>
where
    T: Add<T> + Default + Mul<K>,
    K: Default + Mul<T> + Mul<T, Output = T> + PedersenScalar,
{
    fn default() -> Self {
        PedersenGens {
            g: T::default(),
            h: T::default(),
            _marker: PhantomData,
        }
    }
}

impl<T, K> PedersenGens<T, K>
where
    T: Add<T> + Default + Mul<K> + PedersenGroup,
    K: Default + Mul<T> + Mul<T, Output = T> + PedersenScalar,
{
    fn new<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        PedersenGens {
            g: T::generator(),
            h: K::random(rng) * T::generator(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar;
    use rand_core::OsRng;

    #[test]
    fn test_ristretto() {
        impl PedersenScalar for Scalar {
            fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
                Scalar::random(rng)
            }
        }
        impl PedersenGroup for RistrettoPoint {
            fn generator() -> Self {
                RISTRETTO_BASEPOINT_POINT
            }
            fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
                RistrettoPoint::random(rng)
            }
        }
        let mut rng = OsRng;
        let test_gens: PedersenGens<RistrettoPoint, Scalar> = PedersenGens::new(&mut rng);

        assert_eq!(test_gens.g, RISTRETTO_BASEPOINT_POINT);
    }
}
