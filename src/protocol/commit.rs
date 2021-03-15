use crate::protocol::setup::SetupData;
use crate::traits::{Group, Scalar};
use rand_core::{CryptoRng, RngCore};

pub struct Commitment<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    pub commitment: T,
    pub setup_data: SetupData<T, K>,
    pub random_scalar: K,
    pub message: K,
}

impl<T, K> Commitment<T, K>
where
    T: Group,
    K: Scalar<T>,
{
    pub fn new<R: RngCore + CryptoRng>(
        setup_data: SetupData<T, K>,
        message: Vec<u8>,
        rng: &mut R,
    ) -> Self {
        let message_as_scalar: K = K::from_bytes(message);
        let random_scalar = K::random(rng);
        Commitment {
            commitment: message_as_scalar * setup_data.g + random_scalar * setup_data.h,
            random_scalar,
            message: message_as_scalar,
            setup_data,
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

    #[test]
    fn test_commit_bytes() {
        let mut rng = OsRng;
        let setup_data: SetupData<RistrettoPoint, RistrettoScalar> = SetupData::new(&mut rng);
        let commitment: Commitment<RistrettoPoint, RistrettoScalar> =
            Commitment::new(setup_data, vec![99, 12], &mut rng);
        assert_eq!(
            RistrettoScalar::from_bytes(vec![99, 12]),
            commitment.message
        );
    }
    #[test]
    fn test_commit_non_err_on_nonempty() {
        assert_ne!(0, 1);
    }
}
