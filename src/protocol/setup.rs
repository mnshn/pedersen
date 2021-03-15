use crate::traits::{Group, Scalar};
use rand_core::{CryptoRng, RngCore};
use std::marker::PhantomData;

pub struct SetupData<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    pub g: T,
    pub h: T,
    _marker: PhantomData<K>,
}

impl<T, K> Default for SetupData<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    fn default() -> Self {
        SetupData {
            g: T::default(),
            h: T::default(),
            _marker: PhantomData,
        }
    }
}

impl<T, K> SetupData<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    pub fn new<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        SetupData {
            g: T::generator(),
            h: K::random(rng) * T::generator(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Scalar;
    use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
    use curve25519_dalek::ristretto::RistrettoPoint;
    use curve25519_dalek::scalar::Scalar as RistrettoScalar;
    use rand_core::OsRng;
    use sha2::Sha512;
    impl Scalar<RistrettoPoint> for RistrettoScalar {
        fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
            RistrettoScalar::random(rng)
        }
        fn from_bytes(input: Vec<u8>) -> Self {
            RistrettoScalar::hash_from_bytes::<Sha512>(&input)
        }
    }
    impl Group for RistrettoPoint {
        fn generator() -> Self {
            RISTRETTO_BASEPOINT_POINT
        }
        fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
            RistrettoPoint::random(rng)
        }
    }

    #[test]
    fn test_ristretto() {
        let mut rng = OsRng;
        let test_gens: SetupData<RistrettoPoint, RistrettoScalar> = SetupData::new(&mut rng);

        assert_eq!(test_gens.g, RISTRETTO_BASEPOINT_POINT);
    }
}
